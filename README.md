# schema -- A Generic Schema Derivation for Rust

This crate is pretty simple in concept: it exposes a trait, `Schema`, and a
derive macro for that trait. The trait has a single method, `Schema::schema()`,
that produces a struct of type `syn::DeriveInput` (which this crate re-exports)
that represents the schema for the type on which `schema()` is invoked.

```rust
#[derive(Schema)]
struct MyType {
    a: u32,
}
```

If we `println!("{:#?}", MyType::schema())` we get:

```
DeriveInput {
    attrs: [],
    vis: Inherited,
    ident: Ident(
        MyType,
    ),
    generics: Generics {
        lt_token: None,
        params: [],
        gt_token: None,
        where_clause: None,
    },
    data: Struct(
        DataStruct {
            struct_token: Struct,
            fields: Named(
                FieldsNamed {
                    brace_token: Brace,
                    named: [
                        Field {
                            attrs: [],
                            vis: Inherited,
                            ident: Some(
                                Ident(
                                    a,
                                ),
                            ),
                            colon_token: Some(
                                Colon,
                            ),
                            ty: Path(
                                TypePath {
                                    qself: None,
                                    path: Path {
                                        leading_colon: None,
                                        segments: [
                                            PathSegment {
                                                ident: Ident(
                                                    u32,
                                                ),
                                                arguments: None,
                                            },
                                        ],
                                    },
                                },
                            ),
                        },
                    ],
                },
            ),
            semi_token: None,
        },
    ),
}
```

---

## FAQ

### 1. Ok, so what is this for?

Right. I'm not 100% sure. It seems potentially useful for developing or testing
proc macros. It also seems useful in situations where one might make a proc
macro but can't be bothered to deal with defining a trait on a bunch of base
types.

### 2. Why did you make this?

Fair. I was doing some work with OpenAPI and JSON Schema and found it
surprising I couldn't find a generic schema crate, something that provided a
subset of Java reflection, say.

I found someone squatting on the name `schema` on crates.io. "Aha!" I thought,
"I'll teach that evil squatter a lesson!" So I mailed him and he responded in
the worst possible way: "okay, I sent an invite." Suddenly I was the evil squatter.

Months passed as they do.

I had the odd idea of a derive macro that parsed an item and then teleported
that structure from proc macro context into program context. I usually want an
actual use case to drive a project, but this seemed like a *neat* trick. It led
to weird code like this (i.e. not exactly this):

```rust
impl Teleporter for TokenStream {
    fn teleport(&self) -> TokenStream {
        quote! {
            quote! {
                #self
            }
        }
    }

    //...
}
```

I'm not really sure what this crate is or if it's useful, but it was an
interesting experiment. If you find it useful, have a different idea for how it
should work, or just think `schema` should be a totally different crate: let me
know.

### 3. I'm using it and...

Please tell me about what you're doing with it... you were saying...

### 4. ... I'm seeing an error on some type

Please file an issue. PRs welcome too.
