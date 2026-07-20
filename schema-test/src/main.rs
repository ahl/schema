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
        Schema,
        quote::ToTokens,
        syn::{MacroDelimiter, Meta, MetaNameValue},
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
                    let Meta::List(ll) = &attr.meta else { panic!() };
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

    #[derive(Schema)]
    struct WithFnPtr {
        callback: fn(u32) -> bool,
    }

    #[test]
    fn test_fn_ptr() {
        let _ = WithFnPtr::schema();
    }

    #[derive(Schema)]
    #[repr(i32)]
    enum NegativeDiscriminant {
        A = -1,
        B = 0,
    }

    #[test]
    fn test_negative_discriminant() {
        let _ = NegativeDiscriminant::schema();
    }

    #[derive(Schema)]
    #[repr(u32)]
    enum BinaryDiscriminant {
        A = 1 + 2,
        B = 4,
    }

    #[test]
    fn test_binary_discriminant() {
        let _ = BinaryDiscriminant::schema();
    }

    #[derive(Schema)]
    struct BoolConst<const B: bool = true> {
        a: u32,
    }

    #[test]
    fn test_bool_const() {
        let _ = BoolConst::<true>::schema();
    }

    #[derive(Schema)]
    struct CharConst<const C: char = 'a'> {
        a: u32,
    }

    #[test]
    fn test_char_const() {
        let _ = CharConst::<'a'>::schema();
    }

    #[derive(Schema)]
    #[repr(i32)]
    enum CastDiscriminant {
        A = 'a' as i32,
        B = 0,
    }

    #[test]
    fn test_cast_discriminant() {
        let _ = CastDiscriminant::schema();
    }

    #[derive(Schema)]
    struct ParenArrayLen {
        data: [u8; (4)],
    }

    #[test]
    fn test_paren_array_len() {
        let _ = ParenArrayLen::schema();
    }

    const fn array_size() -> usize {
        4
    }

    #[derive(Schema)]
    struct CallInArrayLen {
        data: [u8; array_size()],
    }

    #[test]
    fn test_call_in_array_len() {
        let _ = CallInArrayLen::schema();
    }

    #[derive(Schema)]
    #[repr(u8)]
    enum ByteDiscriminant {
        A = b'A',
        B = 0,
    }

    #[test]
    fn test_byte_discriminant() {
        let _ = ByteDiscriminant::schema();
    }

    #[derive(Schema)]
    #[repr(i32)]
    enum MethodCallDiscriminant {
        A = 8u32.count_ones() as i32,
        B = 0,
    }

    #[test]
    fn test_method_call_discriminant() {
        let _ = MethodCallDiscriminant::schema();
    }

    const CONST_ARR: [u32; 3] = [1, 2, 3];

    #[derive(Schema)]
    struct WithArrayIndex {
        data: [u8; CONST_ARR[0] as usize],
    }

    #[test]
    fn test_array_index() {
        let _ = WithArrayIndex::schema();
    }

    struct Point {
        x: u32,
    }
    const CONST_POINT: Point = Point { x: 4 };

    #[derive(Schema)]
    struct WithFieldAccess {
        data: [u8; CONST_POINT.x as usize],
    }

    #[test]
    fn test_field_access() {
        let _ = WithFieldAccess::schema();
    }

    #[derive(Schema)]
    #[repr(i32)]
    enum FloatCastDiscriminant {
        A = 3.14 as i32,
        B = 0,
    }

    #[test]
    fn test_float_cast_discriminant() {
        let _ = FloatCastDiscriminant::schema();
    }

    #[derive(Schema)]
    struct WithBlockConst<const N: usize = { 1 + 2 }> {
        _phantom: [u8; N],
    }

    #[test]
    fn test_block_const() {
        let _ = WithBlockConst::<3>::schema();
    }

    #[derive(Schema)]
    #[repr(i32)]
    enum IfDiscriminant {
        A = if true { 1 } else { 0 },
        B = 2,
    }

    #[test]
    fn test_if_discriminant() {
        let _ = IfDiscriminant::schema();
    }

    #[derive(Schema)]
    struct StructLitArrayLen {
        data: [u8; Point { x: 4 }.x as usize],
    }

    #[test]
    fn test_struct_lit_array_len() {
        let _ = StructLitArrayLen::schema();
    }
}
