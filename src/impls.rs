use crate::{Error, Params, creds::Credentials};
use openai_api_rust::{
    Auth, Message, OpenAI, Role,
    chat::{ChatApi, ChatBody},
};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{ForeignItemFn, Signature};

pub(crate) fn impl_foreign_item_fn(
    attr: TokenStream,
    item: ForeignItemFn,
) -> Result<TokenStream, Error> {
    let content = chat_completion(attr, &item.sig)?;
    let tokens: TokenStream = syn::parse_str(&content)?;

    // emit the tokens (ignore warnings since code wont be visible anyway)
    let ForeignItemFn {
        attrs, vis, sig, ..
    } = item;
    Ok(quote!(#(#attrs)* #[allow(warnings)] #vis # sig { #tokens }))
}

fn chat_completion(attr: TokenStream, sig: &Signature) -> Result<String, Error> {
    let Params {
        prompt,
        model,
        temperature,
        top_p,
        presence_penalty,
        frequency_penalty,
        max_tokens,
        ..
    } = Params::new(attr)?;

    // init openai connection
    let credentials = Credentials::from_env()?;
    let openai = OpenAI::new(Auth::new(&credentials.api_key), &credentials.api_url);

    // make the chat request
    let signature = quote!(#sig).to_string();
    let prompt = prompt
        .as_ref()
        .map(String::as_str)
        .unwrap_or_else(|| "No explanation given, figure it out from the function signature");
    let request = ChatBody {
        model: model.unwrap_or_else(|| credentials.api_model),
        max_tokens,
        temperature,
        top_p,
        n: None,
        stream: None,
        stop: None,
        presence_penalty,
        frequency_penalty,
        logit_bias: None,
        user: None,
        messages: vec![Message {
            role: Role::User,
            content: make_message(&signature, prompt),
        }],
    };

    openai
        .chat_completion_create(&request)?
        .choices
        .pop() // any choice is good!
        .and_then(|c| c.message)
        .map(|m| m.content)
        .ok_or(Error::Other("Unable to generate rust code".to_string()))
}

fn make_message(signature: &str, prompt: &str) -> String {
    format!(
        include_str!("prompt.txt"),
        signature = signature,
        prompt = prompt
    )
}
