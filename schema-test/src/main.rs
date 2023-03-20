#![allow(unused)]

use schema::Schema;

#[derive(Schema)]
struct MyType {
    a: u32,
}

fn main() {
    println!("{:#?}", MyType::schema())
}

#[cfg(test)]
mod test {
    use expectorate::assert_contents;
    use schema::{
        quote::ToTokens,
        syn::{MacroDelimiter, Meta, MetaNameValue},
        Schema,
    };

    /// Doc comments are attributes; isn't that grand?!
    #[allow(unused)]
    #[derive(Schema)]
    struct TestStruct {
        a: u32,
        b: String,
        c: Option<u32>,
        d: Vec<Option<Box<u32>>>,
    }

    #[test]
    fn test_struct_output() {
        let actual = format!("{:#?}", TestStruct::schema());
        assert_contents("test_output/struct.out", &actual);
    }

    #[test]
    fn test_struct_fields() {
        let schema = TestStruct::schema();

        assert_eq!(schema.ident.to_string(), "TestStruct");

        let data_struct = match schema.data {
            schema::syn::Data::Struct(s) => s,
            _ => panic!("unexpected data type"),
        };

        let fields_named = match data_struct.fields {
            schema::syn::Fields::Named(n) => n,
            _ => panic!("unexpected fields type"),
        };

        fields_named
            .named
            .iter()
            .zip(["a", "b", "c", "d"].iter())
            .for_each(|(field, name)| {
                assert_eq!(field.ident.as_ref().unwrap().to_string(), name.to_string());
            });
    }
    #[test]
    fn test_struct_attrs() {
        let schema = TestStruct::schema();

        for attr in schema.attrs {
            match attr.path().to_token_stream().to_string().as_str() {
                "allow" => {
                    let Meta::List(ll) = &attr.meta else {
                        panic!()
                    };
                    assert!(matches!(ll.delimiter, MacroDelimiter::Paren(_)));
                    assert_eq!(ll.tokens.to_token_stream().to_string(), "unused")
                }
                "doc" => {
                    let Meta::NameValue(nv) = &attr.meta else {
                        panic!()
                    };
                    assert_eq!(
                        nv.value.to_token_stream().to_string(),
                        r#"" Doc comments are attributes; isn't that grand?!""#,
                    )
                }

                other => panic!("unexpected attr '{}': {:#?}", other, attr),
            }
        }
    }

    #[derive(Schema)]
    enum TestEnum {
        A,
        B(u32),
        C(u32, String),
        D { a: u32 },
    }

    #[test]
    fn test_enum() {
        let actual = format!("{:#?}", TestEnum::schema());
        assert_contents("test_output/enum.out", &actual);
    }

    #[derive(Schema)]
    #[repr(C)]
    union TestUnion {
        f1: u32,
        f2: f32,
    }

    #[test]
    fn test_union() {
        let actual = format!("{:#?}", TestUnion::schema());
        assert_contents("test_output/union.out", &actual);
    }

    #[derive(Schema)]
    struct FancyStruct<'a, T: ToString, U, const N: usize>
    where
        U: ToString,
    {
        /// mind your tees and ewes
        tee: &'a T,
        ewes: [U; N],
    }

    #[test]
    fn test_fancy_struct_output() {
        let actual = format!("{:#?}", FancyStruct::<'static, String, String, 7>::schema());
        assert_contents("test_output/fancy_struct.out", &actual);
    }
}
