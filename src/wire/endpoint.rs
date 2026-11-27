use reqwest::Method;

use crate::net::client::HttpClient;
use crate::support::consts::net::{API_URL, BASE_URL, MODELS_PATH, OPTIONS_PATH, SESSION_PATH};
use crate::support::error::{AppError, AppResult};
use crate::support::log::{lg, ly};
use crate::wire::shape::{ModelsReply, OptionsReply, SessionReply};

pub async fn probe(client: &HttpClient) -> AppResult<()> {
    let options = client
        .send(
            Method::GET,
            &format!("{}{}", BASE_URL, OPTIONS_PATH),
            None,
            &[],
        )
        .await?;
    let status = options.status().as_u16();
    let text = options.text().await.unwrap_or_default();
    if status != 200 {
        return Err(AppError::wire(format!(
            "the options route answered with status {}",
            status
        )));
    }
    let reply: OptionsReply = serde_json::from_str(&text)
        .map_err(|_| AppError::wire("the options reply could not be read"))?;
    if reply.wallets.solana {
        lg("The options route accepts Solana wallet sign in");
    } else {
        ly("The options route has wallet sign in turned off");
    }
    if reply.email {
        ly("The options route also advertises mailbox sign in");
    }
    let anonymous = session(client, "").await;
    match anonymous {
        Ok(_) => ly("The session route answers without a cookie ready"),
        Err(_) => lg("The session route needs a cookie before it answers"),
    }
    Ok(())
}

pub async fn session(client: &HttpClient, cookie: &str) -> AppResult<SessionReply> {
    let response = client
        .send(
            Method::GET,
            &format!("{}{}", BASE_URL, SESSION_PATH),
            None,
            &[("cookie", cookie)],
        )
        .await?;
    let status = response.status().as_u16();
    let text = response.text().await.unwrap_or_default();
    if status != 200 {
        return Err(AppError::wire(format!(
            "the session route answered with status {}",
            status
        )));
    }
    serde_json::from_str(&text).map_err(|_| AppError::wire("the session reply could not be read"))
}

pub async fn models(client: &HttpClient, key: &str) -> AppResult<usize> {
    let bearer = format!("Bearer {}", key);
    let response = client
        .send(
            Method::GET,
            &format!("{}{}", API_URL, MODELS_PATH),
            None,
            &[("authorization", bearer.as_str())],
        )
        .await?;
    let status = response.status().as_u16();
    let text = response.text().await.unwrap_or_default();
    if status != 200 {
        return Err(AppError::wire(format!(
            "the models route answered with status {}",
            status
        )));
    }
    let reply: ModelsReply = serde_json::from_str(&text)
        .map_err(|_| AppError::wire("the models reply could not be read"))?;
    Ok(reply.data.len())
}
