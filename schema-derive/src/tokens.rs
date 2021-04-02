use crate::Teleporter;
use proc_macro2::TokenStream;
use quote::quote;
use syn::token::*;

macro_rules! make_token {
    ($token:ident) => {
        impl Teleporter for $token {
            fn teleport(&self) -> TokenStream {
                quote! {
                    schema::syn::token::$token::default()
                }
            }

            fn name() -> &'static str {
                stringify!(syn::token::$token)
            }
        }
    };
}

make_token!(Add);
make_token!(And);
make_token!(As);
make_token!(Bang);
make_token!(Brace);
make_token!(Bracket);
make_token!(Colon);
make_token!(Colon2);
make_token!(Comma);
make_token!(Const);
make_token!(Crate);
make_token!(Dot3);
make_token!(Dyn);
make_token!(Enum);
make_token!(Eq);
make_token!(Extern);
make_token!(Fn);
make_token!(For);
make_token!(Group);
make_token!(Gt);
make_token!(Impl);
make_token!(In);
make_token!(Lt);
make_token!(Mut);
make_token!(Paren);
make_token!(Pound);
make_token!(Pub);
make_token!(Question);
make_token!(RArrow);
make_token!(Semi);
make_token!(Star);
make_token!(Struct);
make_token!(Underscore);
make_token!(Union);
make_token!(Unsafe);
make_token!(Where);
