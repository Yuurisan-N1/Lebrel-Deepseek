use crate::net::client::HttpClient;
use crate::support::cfg::loader;
use crate::support::consts::app::SESSION_FILE;
use crate::support::error::{AppError, AppResult};
use crate::support::log::{gap, lg, ly};
use crate::wire::endpoint;
use crate::wire::shape::StoredState;
use std::time::Duration;

pub async fn cycle() -> AppResult<()> {
    let root = std::env::current_dir().unwrap_or_else(|_| ".".into());
    let setup = loader::load(&root)?;
    let session_path = root.join(SESSION_FILE);
    let text = std::fs::read_to_string(&session_path)
        .map_err(|_| AppError::config("session.json is missing so create accounts first"))?;
    let state: StoredState = serde_json::from_str(&text)
        .map_err(|_| AppError::config("session.json could not be read"))?;
    if state.accounts.is_empty() {
        return Err(AppError::config("session.json holds no account to run"));
    }

    let mut round = 0usize;
    loop {
        ly(&format!("Starting automation cycle number {}", round + 1));
        for (index, account) in state.accounts.iter().enumerate() {
            if index > 0 {
                gap();
            }
            let route = setup.proxy_for(index);
            if let Some(proxy) = route {
                ly(&format!("Using proxy {}", proxy.masked()));
            }
            let client = HttpClient::new(route)?;
            match endpoint::models(&client, "").await {
                Ok(count) => lg(&format!(
                    "Account {} sees {} models and stays alive",
                    account.key_prefix, count
                )),
                Err(err) => ly(&format!(
                    "Account {} did not answer because of {}",
                    account.key_prefix, err
                )),
            }
        }
        round += 1;
        countdown(setup.config.sleep_seconds()).await;
        crate::support::banner::print_banner();
    }
}

async fn countdown(seconds: u64) {
    let mut left = seconds;
    while left > 0 {
        crate::support::log::live(
            crate::support::log::Y,
            &format!("Next cycle starts in {}", clock(left)),
        );
        tokio::time::sleep(Duration::from_secs(1)).await;
        left -= 1;
    }
    crate::support::log::clear_live();
}

#[allow(dead_code)]
fn clock(seconds: u64) -> String {
    format!(
        "{:02}:{:02}:{:02}",
        seconds / 3600,
        (seconds % 3600) / 60,
        seconds % 60
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_clock_pads_every_part() {
        assert_eq!(clock(3661), "01:01:01");
        assert_eq!(clock(59), "00:00:59");
    }
}
