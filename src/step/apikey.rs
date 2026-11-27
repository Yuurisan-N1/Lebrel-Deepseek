use reqwest::Method;

use crate::net::client::HttpClient;
use crate::support::consts::app::KEY_NAME;
use crate::support::consts::net::{BASE_URL, KEYS_PATH};
use crate::support::device::Device;
use crate::support::error::{AppError, AppResult};
use crate::wire::shape::KeyReply;

pub struct Issued {
    pub id: String,
    pub prefix: String,
    pub secret: String,
}

pub async fn create(client: &HttpClient, device: &Device, cookie: &str) -> AppResult<Issued> {
    let payload = serde_json::json!({ "name": KEY_NAME });
    let extra = [
        ("cookie", cookie),
        ("origin", BASE_URL),
        ("referer", "https://lebrel.ai/dashboard"),
        ("accept-language", device.locale.as_str()),
    ];
    let response = client
        .send(
            Method::POST,
            &format!("{}{}", BASE_URL, KEYS_PATH),
            Some(payload.to_string()),
            &extra,
        )
        .await?;
    let status = response.status().as_u16();
    let text = response.text().await.unwrap_or_default();
    if status != 200 && status != 201 {
        return Err(AppError::wire(format!(
            "the key could not be created with status {}",
            status
        )));
    }
    let reply: KeyReply = serde_json::from_str(&text)
        .map_err(|_| AppError::wire("the key reply could not be read"))?;
    Ok(Issued {
        id: reply.key.id,
        prefix: reply.key.prefix,
        secret: reply.key.secret,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_issued_key_keeps_the_prefix_apart_from_the_secret() {
        let issued = Issued {
            id: "key_1".into(),
            prefix: "leb_live_aa".into(),
            secret: "leb_live_aa_bb".into(),
        };
        assert!(issued.secret.starts_with(&issued.prefix));
    }
}
