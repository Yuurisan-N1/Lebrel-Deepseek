use std::path::Path;

use chrono::Utc;

use crate::net::client::HttpClient;
use crate::step::{apikey, signin, wallet::Wallet};
use crate::support::cfg::loader::Setup;
use crate::support::consts::app::{APKEY_FILE, SESSION_FILE};
use crate::support::device::Device;
use crate::support::error::{AppError, AppResult};
use crate::support::log::{lg, ly};
use crate::wire::shape::{StoredAccount, StoredState};

pub async fn enroll_one(
    setup: &Setup,
    index: usize,
    client: &HttpClient,
    device: &Device,
) -> AppResult<StoredAccount> {
    let _ = index;
    let wallet = Wallet::generate()?;
    let masked = mask_address(&wallet.address);
    ly(&format!("Using wallet {}", wallet.address));

    let (challenge_id, message) = signin::begin(client, device, &wallet).await?;
    let signed = signin::verify(client, device, &wallet, &challenge_id, &message).await?;
    lg(&format!(
        "Wallet {} signed in with account {}",
        masked, signed.account_id
    ));

    let issued = apikey::create(client, device, &signed.cookie).await?;
    lg(&format!(
        "API key {} was issued for wallet {}",
        issued.prefix, masked
    ));

    let record = StoredAccount {
        address: wallet.address.clone(),
        secret: wallet.secret_hex(),
        account_id: signed.account_id,
        device_id: device.device_id.clone(),
        user_agent: device.user_agent.clone(),
        locale: device.locale.clone(),
        key_id: issued.id,
        key_prefix: issued.prefix,
        created_at: Utc::now().to_rfc3339(),
    };

    store(&setup.root, &record, &issued.secret)?;
    Ok(record)
}

pub fn store(root: &Path, record: &StoredAccount, secret: &str) -> AppResult<()> {
    let key_path = root.join(APKEY_FILE);
    let mut lines = match std::fs::read_to_string(&key_path) {
        Ok(text) => text,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => String::new(),
        Err(err) => {
            return Err(AppError::config(format!(
                "{} could not be read: {}",
                APKEY_FILE, err
            )))
        }
    };
    if !lines.is_empty() && !lines.ends_with('\n') {
        lines.push('\n');
    }
    lines.push_str(secret);
    lines.push('\n');
    std::fs::write(&key_path, lines)
        .map_err(|err| AppError::config(format!("{} could not be written: {}", APKEY_FILE, err)))?;

    let session_path = root.join(SESSION_FILE);
    let mut state: StoredState = match std::fs::read_to_string(&session_path) {
        Ok(text) => serde_json::from_str(&text).unwrap_or_default(),
        Err(_) => StoredState::default(),
    };
    state.accounts.push(StoredAccount {
        address: record.address.clone(),
        secret: record.secret.clone(),
        account_id: record.account_id.clone(),
        device_id: record.device_id.clone(),
        user_agent: record.user_agent.clone(),
        locale: record.locale.clone(),
        key_id: record.key_id.clone(),
        key_prefix: record.key_prefix.clone(),
        created_at: record.created_at.clone(),
    });
    let text = serde_json::to_string_pretty(&state)
        .map_err(|err| AppError::config(format!("{} could not be built: {}", SESSION_FILE, err)))?;
    std::fs::write(&session_path, text)
        .map_err(|err| AppError::config(format!("{} could not be written: {}", SESSION_FILE, err)))
}

pub fn mask_address(address: &str) -> String {
    let chars: Vec<char> = address.chars().collect();
    if chars.len() < 12 {
        return address.to_string();
    }
    let head: String = chars[..5].iter().collect();
    let tail: String = chars[chars.len() - 5..].iter().collect();
    format!("{}*{}", head, tail)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scratch(name: &str) -> std::path::PathBuf {
        let dir =
            std::env::temp_dir().join(format!("lebrel-store-{}-{}", std::process::id(), name));
        let _ = std::fs::create_dir_all(&dir);
        dir
    }

    fn record(address: &str) -> StoredAccount {
        StoredAccount {
            address: address.into(),
            secret: "aa".into(),
            account_id: "acc_1".into(),
            device_id: "dev".into(),
            user_agent: "ua".into(),
            locale: "en-US".into(),
            key_id: "key_1".into(),
            key_prefix: "leb_live_aa".into(),
            created_at: "2026-01-01T00:00:00Z".into(),
        }
    }

    #[test]
    fn the_address_shows_five_and_five() {
        let masked = mask_address("7WmsHVMqwuJMMgo1aPiYFWrCLiJAjWMZNzJWnaZv7QEB");
        assert_eq!(masked, "7WmsH*v7QEB");
    }

    #[test]
    fn a_key_is_appended_on_its_own_line() {
        let dir = scratch("keys");
        store(&dir, &record("a"), "leb_live_one").unwrap();
        store(&dir, &record("b"), "leb_live_two").unwrap();
        let text = std::fs::read_to_string(dir.join(APKEY_FILE)).unwrap();
        let lines: Vec<&str> = text.lines().collect();
        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0], "leb_live_one");
        assert_eq!(lines[1], "leb_live_two");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn the_session_keeps_every_account() {
        let dir = scratch("session");
        store(&dir, &record("a"), "k1").unwrap();
        store(&dir, &record("b"), "k2").unwrap();
        let text = std::fs::read_to_string(dir.join(SESSION_FILE)).unwrap();
        let state: StoredState = serde_json::from_str(&text).unwrap();
        assert_eq!(state.accounts.len(), 2);
        assert_eq!(state.accounts[0].secret, "aa");
        let _ = std::fs::remove_dir_all(&dir);
    }
}
