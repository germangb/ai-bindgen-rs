extern crate proc_macro;

use crate::{error::Error, utils::Params};
use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemForeignMod;

mod creds;
mod error;
mod impls;
mod utils;

/// A processor macro for extern blocks and function items within. The annotated
/// functions will have their bodies implemented by OpenAI (or comparible)
/// service according to the supplied prompt and other model parameters.
///
/// # Example
///
/// ```
/// use ai_bindgen::ai;

/// #[ai]
/// extern "C" {
///     #[ai(prompt = "return the n-th prime number, please")]
///     fn prime(n: i32) -> i32;
/// }

/// fn main() {
///     println!("The 15th prime number is {}", prime(15)); // 47 (hopefully)
/// }
/// ```
#[proc_macro_attribute]
pub fn ai(attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    ai2(TokenStream::from(attr), TokenStream::from(item)).into()
}

fn ai2(attr: TokenStream, item: TokenStream) -> TokenStream {
    if let Ok(ItemForeignMod { items, .. }) = syn::parse2(item.clone()) {
        if !attr.is_empty() {
            return Error::InvalidUsage("Attribute parameters are not suppored in extern block.")
                .into_syn(attr)
                .into_compile_error();
        }

        // emit only the inner items, the "extern ..." tokens are discarded
        quote!(#(#items)*).into()
    } else if let Ok(foreign_fn) = syn::parse2(item.clone()) {
        match impls::impl_foreign_item_fn(attr, foreign_fn) {
            Err(err) => err.into_syn(item).into_compile_error(),
            Ok(result) => result,
        }
    } else {
        Error::InvalidUsage("The ai macro is not supported on this item")
            .into_syn(item)
            .into_compile_error()
    }
}
