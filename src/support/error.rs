#![allow(dead_code)]
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("{0}")]
    Config(String),
    #[error("{0}")]
    Network(String),
    #[error("{0}")]
    Api(String),
    #[error("{0}")]
    Blocked(String),
    #[error("{0}")]
    Mail(String),
    #[error("wire: {0}")]
    Wire(String),
}

impl AppError {
    pub fn config(message: impl Into<String>) -> AppError {
        AppError::Config(message.into())
    }

    pub fn network(message: impl Into<String>) -> AppError {
        AppError::Network(message.into())
    }

    pub fn api(message: impl Into<String>) -> AppError {
        AppError::Api(message.into())
    }

    pub fn blocked(message: impl Into<String>) -> AppError {
        AppError::Blocked(message.into())
    }

    pub fn wire(message: impl Into<String>) -> AppError {
        AppError::Wire(message.into())
    }

    pub fn mail(message: impl Into<String>) -> AppError {
        AppError::Mail(message.into())
    }
}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> AppError {
        if err.is_timeout() {
            AppError::Network("the request timed out".to_string())
        } else if err.is_connect() {
            AppError::Network("the connection could not be established".to_string())
        } else if err.is_decode() {
            AppError::Network("the reply could not be read".to_string())
        } else {
            AppError::Network("the request failed".to_string())
        }
    }
}

impl From<serde_json::Error> for AppError {
    fn from(_: serde_json::Error) -> AppError {
        AppError::Network("the reply could not be read".to_string())
    }
}

pub type AppResult<T> = Result<T, AppError>;

pub fn refusal(_message: &str) -> String {
    "the server refused the request".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_refusal_is_written_for_a_human() {
        assert_eq!(
            refusal("Bot check failed. Please refresh and try again."),
            "the server refused the request"
        );
    }

    #[test]
    fn an_unknown_refusal_is_still_short() {
        let text = refusal("SOMETHING_NEW");
        assert_eq!(text, "the server refused the request");
    }

    #[test]
    fn an_empty_refusal_still_reads() {
        assert_eq!(refusal("   "), "the server refused the request");
    }

    #[test]
    fn the_kind_names_the_layer() {
        assert_eq!(AppError::api("x").to_string(), "x");
        assert_eq!(AppError::config("x").to_string(), "x");
    }
}
