use crate::field::Field;
use crate::utils;

use darling::{ast::Data, util::Ignored, FromDeriveInput};
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::Result as SynResult;

#[derive(FromDeriveInput, Debug)]
#[darling(attributes(env))]
pub struct Env {
    ident: syn::Ident,
    attrs: Vec<syn::Attribute>,
    generics: syn::Generics,
    data: Data<Ignored, Field>,
}

impl Env {
    pub fn expand(&self) -> SynResult<TokenStream2> {
        assert!(self.is_struct_type());

        let ident = self.ident.to_token_stream();
        let crate_name = utils::get_crate_name();
        let (impl_generics, ty_generics, where_clause) = self.generics.split_for_impl();
        let fields = self.fields()?;

        let ret = quote! {
            impl #impl_generics #crate_name::FromEnv for #ident #ty_generics #where_clause {
                fn from_env() -> std::result::Result<Self, String> {
                    Ok(
                        Self {
                            #(#fields),*
                        }
                    )
                }
            }
        };

        Ok(ret)
    }

    pub fn is_struct_type(&self) -> bool {
        self.data.is_struct()
    }

    pub fn fields(&self) -> SynResult<Vec<TokenStream2>> {
        let mut v = vec![];

        for field in self.data.as_ref().take_struct().unwrap().into_iter() {
            v.push(field.expand()?)
        }
        Ok(v)
    }
}
