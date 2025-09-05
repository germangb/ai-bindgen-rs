use crate::Error;
use std::env::VarError;

const DEFAULT_API_URL: &str = "https://api.openai.com/v1/";

#[derive(Debug)]
pub struct Credentials {
    /// Valie API token
    pub api_key: String,

    /// Name of the model to use by default.
    pub api_model: String,

    /// API endpoint to make the requests to (defaults to https://api.openai.com/v1/).
    pub api_url: String,
}

impl Credentials {
    pub fn from_env() -> Result<Self, Error> {
        let api_key = std::env::var("OPENAI_API_KEY")
            .map_err(|_| "OPENAI_API_KEY environment variable must be defined.")?;

        let api_model = std::env::var("OPENAI_API_MODEL")
            .map_err(|_| "OPENAI_API_MODEL environment variable must be defined.")?;

        let mut api_url = match std::env::var("OPENAI_API_URL") {
            Err(VarError::NotPresent) => DEFAULT_API_URL.to_string(),
            Err(err) => return Err(Error::Env(err)),
            Ok(url) => url,
        };

        // need to add a / at the end so the openai api crate doesn't misconstruct the
        // endpoint
        if !api_url.ends_with('/') {
            api_url.push('/');
        }

        Ok(Self {
            api_key,
            api_model,
            api_url,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::sync::Mutex;

    // Just in case tests are run in parallel, they won't be fighting for the
    // environment variables.
    static MUTEX: Mutex<()> = Mutex::new(());

    #[test]
    fn default_api_url() {
        let _lock = MUTEX.lock().unwrap();

        unsafe {
            std::env::set_var("OPENAI_API_KEY", "foo");
            std::env::set_var("OPENAI_API_MODEL", "bar");
            std::env::remove_var("OPENAI_API_URL");
        }

        let creds = Credentials::from_env().unwrap();

        assert_eq!(DEFAULT_API_URL, &creds.api_url);
        assert_eq!("foo", &creds.api_key);
    }

    #[test]
    fn alt_api_url() {
        let _lock = MUTEX.lock().unwrap();

        unsafe {
            std::env::set_var("OPENAI_API_KEY", "foo");
            std::env::set_var("OPENAI_API_MODEL", "bar");
            std::env::set_var("OPENAI_API_URL", "https://example.com/v1/");
        }

        let creds = Credentials::from_env().unwrap();

        assert_eq!("https://example.com/v1/", &creds.api_url);
        assert_eq!("foo", &creds.api_key);

        unsafe {
            std::env::set_var("OPENAI_API_URL", "https://example.com/v2");
        }

        let creds = Credentials::from_env().unwrap();

        assert_eq!("https://example.com/v2/", &creds.api_url);
    }

    #[test]
    fn no_api_key_error() {
        let _lock = MUTEX.lock().unwrap();

        unsafe {
            std::env::remove_var("OPENAI_API_KEY");
            std::env::set_var("OPENAI_API_MODEL", "bar");
        }

        let creds = Credentials::from_env();

        assert!(matches!(creds, Err(Error::InvalidUsage(_))));
    }

    #[test]
    fn no_api_model_error() {
        let _lock = MUTEX.lock().unwrap();

        unsafe {
            std::env::set_var("OPENAI_API_KEY", "foo");
            std::env::remove_var("OPENAI_API_MODEL");
        }

        let creds = Credentials::from_env();

        assert!(matches!(creds, Err(Error::InvalidUsage(_))));
    }
}
