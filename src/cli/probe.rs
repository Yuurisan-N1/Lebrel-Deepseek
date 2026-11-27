use crate::net::client::HttpClient;
use crate::support::cfg::loader;
use crate::support::error::AppResult;
use crate::support::log::lg;
use crate::wire::endpoint;

pub async fn run() -> AppResult<()> {
    let root = std::env::current_dir().unwrap_or_else(|_| ".".into());
    let setup = loader::load(&root)?;
    let client = HttpClient::new(setup.proxy_for(0))?;
    endpoint::probe(&client).await?;
    lg("The sign in routes answered and the client is ready");
    Ok(())
}
