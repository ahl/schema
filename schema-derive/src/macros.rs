macro_rules! make_struct {
    ($celf:ident, $ty:ty $(, $fields:ident)*) => {
        make_struct!($celf, $ty, $($fields,)*)
    };
    ($celf:ident, $ty:ty $(, $fields:ident)*,) => {
        {
            $(
                let $fields = $celf.$fields.teleport();
            )*
            quote::quote!{
                schema::syn:: $ty {
                    $($fields: #$fields,)*
                }
            }
        }
    };
}

macro_rules! make_enum_match {
    // bare
    ($ty:ident, $variant:ident) => {
        $ty::$variant
    };
    // tuple
    ($ty:ident, $variant:ident ( $( $args:ident ),* )) => {
        $ty::$variant( $( $args ),* )
    };
    // struct
    // TODO
}

macro_rules! make_enum_result {
    // bare
    ($ty:ident, $variant:ident) => {
        quote::quote! { schema::syn :: $ty :: $variant }
    };
    // tuple
    ($ty:ident, $variant:ident ( $( $args:ident ),* )) => {
        {
            let items = vec![
                $(
                    $args.teleport(),
                )*
            ];
            quote::quote! { schema::syn :: $ty :: $variant ( #( #items ),* ) }
        }
    };
    // struct
    // TODO
}

macro_rules! make_enum {
    ($celf:ident, $ty:ident,) => {
        make_enum!($celf, $ty)
    };
    ($celf:ident, $ty:ident, $( $variant:ident $( $args:tt )? ,)+ ) => {
        make_enum!($celf, $ty, $( $variant $( $args )* ),* )
    };
    ($celf:ident, $ty:ident $(, $variant:ident $( $args:tt )? )* ) => {
        match $celf {
            $(
                make_enum_match!($ty, $variant $( $args )*) =>
                    make_enum_result!($ty, $variant $( $args )*),
            )*
            #[allow(unreachable_patterns)]
            unknown =>
                panic!("private variant {} {:?}", stringify!($ty), unknown)
        }
    };
}
