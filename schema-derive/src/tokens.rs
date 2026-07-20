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

make_token!(And);
make_token!(AndAnd);
make_token!(AndEq);
make_token!(As);
make_token!(Brace);
make_token!(Bracket);
make_token!(Caret);
make_token!(CaretEq);
make_token!(Colon);
make_token!(Comma);
make_token!(Const);
make_token!(Crate);
make_token!(DotDotDot);
make_token!(Dyn);
make_token!(Enum);
make_token!(Eq);
make_token!(EqEq);
make_token!(Extern);
make_token!(Fn);
make_token!(For);
make_token!(Ge);
make_token!(Group);
make_token!(Gt);
make_token!(Impl);
make_token!(In);
make_token!(Le);
make_token!(Lt);
make_token!(Minus);
make_token!(MinusEq);
make_token!(Mut);
make_token!(Ne);
make_token!(Not);
make_token!(Or);
make_token!(OrEq);
make_token!(OrOr);
make_token!(Paren);
make_token!(PathSep);
make_token!(Percent);
make_token!(PercentEq);
make_token!(Plus);
make_token!(PlusEq);
make_token!(Pound);
make_token!(Pub);
make_token!(Question);
make_token!(RArrow);
make_token!(Semi);
make_token!(Shl);
make_token!(ShlEq);
make_token!(Shr);
make_token!(ShrEq);
make_token!(Slash);
make_token!(SlashEq);
make_token!(Star);
make_token!(StarEq);
make_token!(Struct);
make_token!(Underscore);
make_token!(Union);
make_token!(Unsafe);
make_token!(Where);
