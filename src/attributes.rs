use crate::error::Error;
use proc_macro2::TokenStream;
use syn::{LitFloat, LitInt, LitStr};

/// The parameters of the [ai] macro.
#[derive(Debug, Default)]
pub struct Attributes {
    /// The string expression passed to the `prompt` attribute.
    pub prompt: Option<String>,

    /// The identifier of the model to use.
    pub model: Option<String>,

    /// Temperature parameter
    pub temperature: Option<f32>,

    /// Top probability mass parameter
    pub top_p: Option<f32>,

    /// Presence penalty parameter
    pub presence_penalty: Option<f32>,

    /// Frequency penalty parameter
    pub frequency_penalty: Option<f32>,

    /// Maximum tokens
    pub max_tokens: Option<i32>,
}

impl Attributes {
    /// Parse the contents of the attribute macro
    pub fn new(attr: TokenStream) -> Result<Self, Error> {
        let mut params = Attributes::default();
        let error = params.parse(attr.into());
        if error.is_empty() {
            Ok(params)
        } else {
            let error = Error::InvalidUsage("Error parsing meta properties")
                .into_syn(TokenStream::from(error));
            Err(Error::from(error))
        }
    }

    // TODO(german) proc_macro is not allowed in test code. Re-implement Params
    // using proc_macro2 instead and re-enable test below
    fn parse(&mut self, attr: proc_macro::TokenStream) -> proc_macro::TokenStream {
        let params_parser = syn::meta::parser(|meta| {
            if meta.path.is_ident("prompt") {
                let lit: LitStr = meta.value()?.parse()?;
                self.prompt = Some(lit.value());
                Ok(())
            } else if meta.path.is_ident("model") {
                let lit: LitStr = meta.value()?.parse()?;
                self.model = Some(lit.value());
                Ok(())
            } else if meta.path.is_ident("temperature") {
                let lit: LitFloat = meta.value()?.parse()?;
                self.temperature = Some(lit.base10_parse()?);
                Ok(())
            } else if meta.path.is_ident("top_p") {
                let lit: LitFloat = meta.value()?.parse()?;
                self.top_p = Some(lit.base10_parse()?);
                Ok(())
            } else if meta.path.is_ident("presence_penalty") {
                let lit: LitFloat = meta.value()?.parse()?;
                self.presence_penalty = Some(lit.base10_parse()?);
                Ok(())
            } else if meta.path.is_ident("frequency_penalty") {
                let lit: LitFloat = meta.value()?.parse()?;
                self.frequency_penalty = Some(lit.base10_parse()?);
                Ok(())
            } else if meta.path.is_ident("max_tokens") {
                let lit: LitInt = meta.value()?.parse()?;
                self.max_tokens = Some(lit.base10_parse()?);
                Ok(())
            } else {
                Err(meta.error("unsupported property"))
            }
        });
        syn::parse_macro_input!(attr with params_parser);
        proc_macro::TokenStream::new() // empty token steam signals no error
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use quote::quote;

    #[test]
    #[ignore]
    fn no_params() {
        let params = Attributes::new(quote!()).unwrap();

        assert_eq!(None, params.model);
        assert_eq!(None, params.prompt);
    }

    #[test]
    #[ignore]
    fn parses_prompt() {
        let params = Attributes::new(quote!(prompt = "hello world")).unwrap();

        assert_eq!(Some("hello world".to_string()), params.prompt);
    }

    #[test]
    #[ignore]
    fn parses_prompt_with_model() {
        let params = Attributes::new(quote!(prompt = "hello world", model = "the-model")).unwrap();

        assert_eq!(Some("hello world".to_string()), params.prompt);
        assert_eq!(Some("the-model".to_string()), params.model);
    }

    #[test]
    #[ignore]
    fn prompt_is_not_string() {
        let params = Attributes::new(quote!(prompt = 42));

        assert!(params.is_err());
    }

    #[test]
    #[ignore]
    fn model_is_not_string() {
        let params = Attributes::new(quote!(model = 42));

        assert!(params.is_err());
    }
}
