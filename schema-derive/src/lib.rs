extern crate proc_macro;

use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

#[macro_use]
mod macros;

mod basic_types;
mod syn_types;
mod tokens;

#[proc_macro_derive(Schema)]
pub fn derive_schema(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    match do_derive_schema(item.into()) {
        Ok(result) => result.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

fn do_derive_schema(item: TokenStream) -> syn::Result<TokenStream> {
    let input = syn::parse2::<DeriveInput>(item)?;

    let output = input.teleport();
    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    Ok(quote! {
        impl #impl_generics Schema for #name #ty_generics #where_clause{
            fn schema() -> schema::DeriveInput {
                #output
            }
        }
    })
}

trait Teleporter {
    fn teleport(&self) -> TokenStream;

    fn name() -> &'static str {
        std::any::type_name::<Self>()
    }
}
