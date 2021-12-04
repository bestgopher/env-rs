mod env;
mod field;
mod utils;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

use darling::FromDeriveInput;

use crate::env::Env;

#[proc_macro_derive(FromEnv, attributes(env))]
pub fn derive(input: TokenStream) -> TokenStream {
    let env: Env = Env::from_derive_input(&parse_macro_input!(input as DeriveInput)).unwrap();
    env.expand().unwrap_or_else(|e| e.to_compile_error()).into()
}
