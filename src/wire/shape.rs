#![allow(dead_code)]
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct NonceRequest {
    pub namespace: String,
    pub address: String,
    pub chain_id: String,
    pub purpose: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NonceReply {
    pub challenge_id: String,
    pub message: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerifyReply {
    pub account: Account,
    pub wallet: Wallet,
}

#[derive(Debug, Deserialize)]
pub struct Account {
    pub id: String,
    pub status: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Wallet {
    pub address: String,
    pub chain_id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionReply {
    pub balance_microusd: Option<i64>,
    pub plan: Option<String>,
    pub priority: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct KeysReply {
    pub keys: Vec<KeyRow>,
}

#[derive(Debug, Deserialize)]
pub struct KeyRow {
    pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct KeyReply {
    pub key: KeySecret,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeySecret {
    pub id: String,
    pub prefix: String,
    pub secret: String,
}

#[derive(Debug, Deserialize)]
pub struct OptionsReply {
    pub email: bool,
    pub wallets: WalletsOn,
}

#[derive(Debug, Deserialize)]
pub struct WalletsOn {
    pub solana: bool,
}

#[derive(Debug, Deserialize)]
pub struct ModelsReply {
    pub data: Vec<ModelRow>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelRow {
    pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct ErrorBody {
    pub error: ErrorInner,
}

#[derive(Debug, Deserialize)]
pub struct ErrorInner {
    pub code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StoredAccount {
    pub address: String,
    pub secret: String,
    pub account_id: String,
    pub device_id: String,
    pub user_agent: String,
    pub locale: String,
    pub key_id: String,
    pub key_prefix: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct StoredState {
    pub accounts: Vec<StoredAccount>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_nonce_reply_parses() {
        let raw = r#"{"challengeId":"chl_1","message":"sign me"}"#;
        let reply: NonceReply = serde_json::from_str(raw).unwrap();
        assert_eq!(reply.challenge_id, "chl_1");
    }

    #[test]
    fn a_key_reply_parses_the_secret() {
        let raw = r#"{"key":{"id":"key_1","prefix":"leb_live_aa","secret":"leb_live_aa_bb"}}"#;
        let reply: KeyReply = serde_json::from_str(raw).unwrap();
        assert_eq!(reply.key.secret, "leb_live_aa_bb");
    }

    #[test]
    fn an_error_body_parses_the_code() {
        let raw = r#"{"error":{"code":"invalid_wallet"}}"#;
        let body: ErrorBody = serde_json::from_str(raw).unwrap();
        assert_eq!(body.error.code, "invalid_wallet");
    }

    #[test]
    fn a_stored_state_round_trips() {
        let mut state = StoredState::default();
        state.accounts.push(StoredAccount {
            address: "abc".into(),
            secret: "def".into(),
            account_id: "acc_1".into(),
            device_id: "dev".into(),
            user_agent: "ua".into(),
            locale: "en-US".into(),
            key_id: "key_1".into(),
            key_prefix: "leb_live_aa".into(),
            created_at: "2026-01-01T00:00:00Z".into(),
        });
        let text = serde_json::to_string(&state).unwrap();
        let again: StoredState = serde_json::from_str(&text).unwrap();
        assert_eq!(again.accounts.len(), 1);
    }
}
