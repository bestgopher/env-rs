use proc_macro2::{
    TokenStream as TokenStream2,
    Span,
};
use syn::{parse_macro_input, Result as SynResult, Error as SynError, DeriveInput, Ident, DataStruct, Data, Fields, FieldsNamed};
use proc_macro::TokenStream;
use proc_macro_crate::{crate_name, FoundCrate};
use quote::{quote, ToTokens};
use syn::spanned::Spanned;


#[proc_macro_derive(Parser)]
pub fn derive(input: TokenStream) -> TokenStream {
    let token = parse_macro_input!(input as DeriveInput);
    build(&token).unwrap_or_else(|e| e.to_compile_error()).into()
}

fn build(token: &DeriveInput) -> SynResult<TokenStream2> {
    let struct_type = get_struct(token)?;
    let ident = token.ident.to_token_stream();
    let crate_name = get_crate_name();
    let (impl_generics, ty_generics, where_clause) = token.generics.split_for_impl();
    let fields = fields(struct_type)?;

    let ret = quote! {
        impl #impl_generics #crate_name::Parser for #ident #ty_generics #where_clause {
            fn parse() -> std::result::Result<Self, String> {
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

fn fields(token: &DataStruct) -> SynResult<Vec<TokenStream2>> {
    let crate_name = get_crate_name();
    if let Fields::Named(FieldsNamed { ref named, .. }) = token.fields {
        Ok(named
            .iter()
            .map(
                |x| {
                    let ident = x.ident.as_ref().unwrap();
                    let field_name = ident.to_string();
                    quote!(#ident: #crate_name::var(#field_name)?)
                })
            .collect())
    } else {
        Err(SynError::new(Span::call_site(), "must be field-named struct"))
    }
}

fn get_struct(token: &DeriveInput) -> SynResult<&DataStruct> {
    if let Data::Struct(ref d) = token.data {
        Ok(d)
    } else {
        Err(SynError::new(token.span(), "not a struct object"))
    }
}

fn get_crate_name() -> TokenStream2 {
    let name = match crate_name("env_rs") {
        Ok(FoundCrate::Name(name)) => name,
        Ok(FoundCrate::Itself) => "crate".to_string(),
        Err(_) => "env_rs".to_string(),
    };

    let name = Ident::new(name.as_str(), Span::call_site());
    quote!(#name)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
