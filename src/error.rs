use quote::ToTokens;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Invalid usage: {0}")]
    InvalidUsage(&'static str),

    #[error("Rust parser error: {0}")]
    Syn(#[from] syn::Error),

    #[error("OpenAI API error: {0}")]
    OpenAI(openai_api_rust::Error),

    #[error("Environment error: {0}")]
    Env(#[from] std::env::VarError),

    #[error("Other error: {0}")]
    Other(String),
}

impl Error {
    pub fn into_syn<T: ToTokens>(self, tokens: T) -> syn::Error {
        if let Error::Syn(error) = self {
            // preserve the tokens of the syn Error
            error
        } else {
            syn::Error::new_spanned(tokens, self)
        }
    }
}

impl From<openai_api_rust::Error> for Error {
    fn from(value: openai_api_rust::Error) -> Self {
        Self::OpenAI(value)
    }
}

impl From<&'static str> for Error {
    fn from(value: &'static str) -> Self {
        Self::InvalidUsage(value)
    }
}
