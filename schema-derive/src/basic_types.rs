use proc_macro2::TokenStream;
use quote::quote;

use crate::Teleporter;

impl<T> Teleporter for std::boxed::Box<T>
where
    T: Teleporter,
{
    fn teleport(&self) -> TokenStream {
        let t = self.as_ref().teleport();
        quote! {
            Box::new(#t)
        }
    }

    fn name() -> &'static str {
        "std::boxed::Box"
    }
}

impl Teleporter for TokenStream {
    fn teleport(&self) -> TokenStream {
        quote! {
            schema::quote::quote! {
                #self
            }
        }
    }

    fn name() -> &'static str {
        "proc_macro2::TokenStream"
    }
}

impl<T> Teleporter for Vec<T>
where
    T: Teleporter,
{
    fn teleport(&self) -> proc_macro2::TokenStream {
        let tt = self.iter().map(|t| t.teleport());
        quote! {
            vec![
                #(
                    #tt,
                )*
            ]
        }
    }

    fn name() -> &'static str {
        "std::vec::Vec"
    }
}

impl<T> Teleporter for Option<T>
where
    T: Teleporter,
{
    fn teleport(&self) -> proc_macro2::TokenStream {
        match self {
            None => quote! {
                None
            },
            Some(t) => {
                let tt = t.teleport();
                quote! { Some(#tt) }
            }
        }
    }

    fn name() -> &'static str {
        "std::Option"
    }
}

impl Teleporter for usize {
    fn teleport(&self) -> TokenStream {
        quote! { #self }
    }

    fn name() -> &'static str {
        "usize"
    }
}

impl<T, U> Teleporter for (T, U)
where
    T: Teleporter,
    U: Teleporter,
{
    fn teleport(&self) -> proc_macro2::TokenStream {
        let (t, u) = (self.0.teleport(), self.1.teleport());
        quote! {
            (#t, #u)
        }
    }

    fn name() -> &'static str {
        todo!()
    }
}
