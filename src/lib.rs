extern crate proc_macro;

use crate::{attributes::Attributes, error::Error};
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{ForeignItemFn, ItemForeignMod};

mod attributes;
mod credentials;
mod error;
mod impls;

trait Transform: ToTokens + Clone {
    fn try_transform(self, attr: TokenStream) -> Result<TokenStream, Error>;

    /// Wraps the call to [`Transform::try_transform`] and returns a
    /// `TokenStream` for a compile error if the result was an error.
    fn transform(self, attr: TokenStream) -> TokenStream {
        match Self::try_transform(self.clone(), attr) {
            Ok(result) => result,
            Err(err) => err.into_syn(self).into_compile_error(),
        }
    }
}

/// A processor macro for extern blocks and function items within. The annotated
/// functions will have their bodies implemented by OpenAI (or comparible)
/// service according to the supplied prompt and other model parameters.
///
/// # Example
///
/// ```
/// use ai_bindgen::ai;
///
/// #[ai]
/// extern "C" {
///     #[ai(prompt = "return the n-th prime number, please")]
///     fn prime(n: i32) -> i32;
/// }
///
/// fn main() {
///     println!("The 15th prime number is {}", prime(15)); // 47 (hopefully)
/// }
/// ```
#[proc_macro_attribute]
pub fn ai(attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let attr = TokenStream::from(attr);
    let item = TokenStream::from(item);

    // parse and transform tokens
    let result = if let Ok(item) = syn::parse2(item.clone()) {
        ItemForeignMod::transform(item, attr)
    } else if let Ok(item) = syn::parse2(item.clone()) {
        ForeignItemFn::transform(item, attr)
    } else {
        Error::InvalidUsage("The ai macro is not supported on this item")
            .into_syn(item)
            .into_compile_error()
    };

    result.into()
}
