use proc_macro2::TokenStream;
use quote::quote;
use syn::*;
use syn::{Path, parse_str, punctuated::Punctuated};

use crate::Teleporter;

impl Teleporter for DeriveInput {
    fn teleport(&self) -> proc_macro2::TokenStream {
        make_struct!(self, DeriveInput, attrs, vis, ident, generics, data)
    }
}

impl Teleporter for Attribute {
    fn teleport(&self) -> proc_macro2::TokenStream {
        make_struct!(self, Attribute, pound_token, style, bracket_token, meta)
    }
}

impl Teleporter for AttrStyle {
    fn teleport(&self) -> proc_macro2::TokenStream {
        make_enum!(self, AttrStyle, Inner(bang), Outer)
    }
}

impl Teleporter for Visibility {
    fn teleport(&self) -> proc_macro2::TokenStream {
        make_enum!(
            self,
            Visibility,
            Public(vis_public),
            Restricted(vis_restricted),
            Inherited
        )
    }
}

impl Teleporter for Ident {
    fn teleport(&self) -> proc_macro2::TokenStream {
        let ident = self.to_string();
        quote! {
            schema::syn::Ident::new(#ident, schema::proc_macro2::Span::call_site())
        }
    }
}

impl Teleporter for Lifetime {
    fn teleport(&self) -> proc_macro2::TokenStream {
        let ident = format!("'{}", self.ident.to_string());
        quote! {
            schema::syn::Lifetime::new(#ident, schema::proc_macro2::Span::call_site())
        }
    }
}

impl Teleporter for Generics {
    fn teleport(&self) -> proc_macro2::TokenStream {
        make_struct!(self, Generics, lt_token, params, gt_token, where_clause)
    }
}

impl Teleporter for Data {
    fn teleport(&self) -> proc_macro2::TokenStream {
        make_enum!(
            self,
            Data,
            Struct(data_struct),
            Enum(data_enum),
            Union(data_union),
        )
    }
}

// `std::any::type_name` reports the definition site (e.g. `syn::ty::TypePath`),
// but syn re-exports these types at the crate root (`syn::TypePath`). Collapse
// `syn::<internal>::...::<Leaf>` to `syn::<Leaf>` so the emitted path resolves.
fn normalize_syn_path(name: &str) -> String {
    if let Some(rest) = name.strip_prefix("syn::") {
        if let Some(pos) = rest.rfind("::") {
            return format!("syn::{}", &rest[pos + 2..]);
        }
    }
    name.to_string()
}

impl<T, P> Teleporter for Punctuated<T, P>
where
    T: crate::Teleporter,
    P: crate::Teleporter,
{
    fn teleport(&self) -> proc_macro2::TokenStream {
        let tname = format!("schema::{}", normalize_syn_path(T::name()));
        let pname = format!("schema::{}", normalize_syn_path(P::name()));
        let t = parse_str::<Path>(tname.as_str()).unwrap();
        let p = parse_str::<Path>(pname.as_str()).unwrap();

        let values = self.iter().map(|t| t.teleport());
        quote! {
            (vec![
                #( #values, )*
            ] as Vec<#t>).into_iter().collect::<schema::syn::punctuated::Punctuated::<#t, #p>>()
        }
    }
}

impl Teleporter for VisRestricted {
    fn teleport(&self) -> proc_macro2::TokenStream {
        make_struct!(self, VisRestricted, pub_token, paren_token, in_token, path)
    }
}

impl Teleporter for DataStruct {
    fn teleport(&self) -> proc_macro2::TokenStream {
        make_struct!(self, DataStruct, struct_token, fields, semi_token)
    }
}

impl Teleporter for DataEnum {
    fn teleport(&self) -> proc_macro2::TokenStream {
        make_struct!(self, DataEnum, enum_token, brace_token, variants)
    }
}

impl Teleporter for Variant {
    fn teleport(&self) -> proc_macro2::TokenStream {
        make_struct!(self, Variant, attrs, ident, fields, discriminant)
    }
}

impl Teleporter for DataUnion {
    fn teleport(&self) -> TokenStream {
        make_struct!(self, DataUnion, union_token, fields)
    }
}

impl Teleporter for Fields {
    fn teleport(&self) -> proc_macro2::TokenStream {
        make_enum!(
            self,
            Fields,
            Named(fields_named),
            Unnamed(fields_unnamed),
            Unit
        )
    }
}

impl Teleporter for FieldsNamed {
    fn teleport(&self) -> proc_macro2::TokenStream {
        make_struct!(self, FieldsNamed, brace_token, named)
    }
}
impl Teleporter for FieldsUnnamed {
    fn teleport(&self) -> TokenStream {
        make_struct!(self, FieldsUnnamed, paren_token, unnamed)
    }
}

impl Teleporter for GenericParam {
    fn teleport(&self) -> proc_macro2::TokenStream {
        make_enum!(
            self,
            GenericParam,
            Type(type_param),
            Lifetime(lifetime_def),
            Const(const_param)
        )
    }
}
impl Teleporter for TypeParam {
    fn teleport(&self) -> proc_macro2::TokenStream {
        make_struct!(self, TypeParam, attrs, ident, colon_token, bounds, default)
    }
}
impl Teleporter for TypeParamBound {
    fn teleport(&self) -> proc_macro2::TokenStream {
        make_enum!(self, TypeParamBound, Trait(trait_bound), Lifetime(lifetime))
    }
}

impl Teleporter for TraitBound {
    fn teleport(&self) -> proc_macro2::TokenStream {
        make_struct!(
            self,
            TraitBound,
            paren_token,
            modifiers,
            lifetimes,
            maybe,
            path,
        )
    }
}

impl Teleporter for TraitBoundModifiers {
    fn teleport(&self) -> TokenStream {
        // As a non-exhaustive struct, this is our only way to construct it.
        quote! {
            Default::default()
        }
    }
}

impl Teleporter for ConstParam {
    fn teleport(&self) -> proc_macro2::TokenStream {
        make_struct!(
            self,
            ConstParam,
            attrs,
            const_token,
            ident,
            colon_token,
            ty,
            default,
        )
    }
}

impl Teleporter for Field {
    fn teleport(&self) -> proc_macro2::TokenStream {
        make_struct!(
            self,
            Field,
            attrs,
            vis,
            modifiers,
            ident,
            colon_token,
            ty,
            default,
        )
    }
}

impl Teleporter for FieldModifiers {
    fn teleport(&self) -> TokenStream {
        // As a non-exhaustive struct, this is our only way to construct it.
        quote! {
            Default::default()
        }
    }
}

impl Teleporter for Type {
    fn teleport(&self) -> TokenStream {
        make_enum!(
            self,
            Type,
            Array(type_array),
            FnPtr(type_fn_ptr),
            Group(type_group),
            ImplTrait(type_impl_trait),
            Infer(type_infer),
            Macro(type_macro),
            Never(type_never),
            Paren(type_paren),
            Path(type_path),
            Ptr(type_ptr),
            Reference(type_reference),
            Slice(type_slice),
            TraitObject(type_trait_object),
            Tuple(type_tuple),
            Verbatim(token_stream),
        )
    }
}

impl Teleporter for TypeArray {
    fn teleport(&self) -> TokenStream {
        make_struct!(self, TypeArray, attrs, bracket_token, elem, semi_token, len)
    }
}

impl Teleporter for BoundLifetimes {
    fn teleport(&self) -> proc_macro2::TokenStream {
        make_struct!(
            self,
            BoundLifetimes,
            for_token,
            lt_token,
            lifetimes,
            gt_token
        )
    }
}

impl Teleporter for Abi {
    fn teleport(&self) -> TokenStream {
        make_struct!(self, Abi, extern_token, name)
    }
}

impl Teleporter for Variadic {
    fn teleport(&self) -> TokenStream {
        make_struct!(self, Variadic, attrs, dots)
    }
}

impl Teleporter for ReturnType {
    fn teleport(&self) -> TokenStream {
        make_enum!(self, ReturnType, Type(rarrow, box_type), Default)
    }
}

impl Teleporter for TypeTuple {
    fn teleport(&self) -> TokenStream {
        make_struct!(self, TypeTuple, attrs, paren_token, elems)
    }
}

impl Teleporter for WhereClause {
    fn teleport(&self) -> proc_macro2::TokenStream {
        make_struct!(self, WhereClause, where_token, predicates)
    }
}

impl Teleporter for WherePredicate {
    fn teleport(&self) -> proc_macro2::TokenStream {
        make_enum!(
            self,
            WherePredicate,
            Lifetime(predicate_lifetime),
            Type(predicate_type),
        )
    }
}

impl Teleporter for PredicateType {
    fn teleport(&self) -> TokenStream {
        make_struct!(
            self,
            PredicateType,
            attrs,
            lifetimes,
            bounded_ty,
            colon_token,
            bounds,
        )
    }
}

impl Teleporter for PredicateLifetime {
    fn teleport(&self) -> TokenStream {
        make_struct!(
            self,
            PredicateLifetime,
            attrs,
            lifetime,
            colon_token,
            bounds
        )
    }
}

impl Teleporter for TypeTraitObject {
    fn teleport(&self) -> TokenStream {
        make_struct!(self, TypeTraitObject, attrs, dyn_token, bounds)
    }
}

impl Teleporter for TypeReference {
    fn teleport(&self) -> TokenStream {
        make_struct!(
            self,
            TypeReference,
            attrs,
            and_token,
            lifetime,
            mutability,
            elem,
        )
    }
}

impl Teleporter for TypeSlice {
    fn teleport(&self) -> TokenStream {
        make_struct!(self, TypeSlice, attrs, bracket_token, elem)
    }
}

impl Teleporter for TypePtr {
    fn teleport(&self) -> proc_macro2::TokenStream {
        make_struct!(self, TypePtr, attrs, star_token, mutability, elem)
    }
}

impl Teleporter for TypePath {
    fn teleport(&self) -> proc_macro2::TokenStream {
        make_struct!(self, TypePath, attrs, qself, path)
    }
}

impl Teleporter for QSelf {
    fn teleport(&self) -> proc_macro2::TokenStream {
        make_struct!(self, QSelf, lt_token, ty, position, as_token, gt_token)
    }
}

impl Teleporter for Path {
    fn teleport(&self) -> proc_macro2::TokenStream {
        make_struct!(self, Path, leading_colon, segments)
    }
}

impl Teleporter for PathSegment {
    fn teleport(&self) -> proc_macro2::TokenStream {
        make_struct!(self, PathSegment, ident, arguments)
    }
}

impl Teleporter for PathArguments {
    fn teleport(&self) -> TokenStream {
        make_enum!(
            self,
            PathArguments,
            AngleBracketed(angle_bracketed_generic_arguments),
            Parenthesized(parenthesized_generic_arguments),
            None
        )
    }
}

impl Teleporter for AngleBracketedGenericArguments {
    fn teleport(&self) -> proc_macro2::TokenStream {
        make_struct!(
            self,
            AngleBracketedGenericArguments,
            colon2_token,
            lt_token,
            args,
            gt_token
        )
    }
}

impl Teleporter for GenericArgument {
    fn teleport(&self) -> proc_macro2::TokenStream {
        make_enum!(
            self,
            GenericArgument,
            Lifetime(lifetime),
            Type(ttype),
            Const(expr),
            AssocType(assoc_type),
            AssocConst(assoc_const),
            Constraint(constraint),
        )
    }
}

impl Teleporter for Constraint {
    fn teleport(&self) -> proc_macro2::TokenStream {
        make_struct!(self, Constraint, ident, generics, colon_token, bounds)
    }
}

impl Teleporter for ParenthesizedGenericArguments {
    fn teleport(&self) -> TokenStream {
        make_struct!(
            self,
            ParenthesizedGenericArguments,
            paren_token,
            inputs,
            output
        )
    }
}

impl Teleporter for TypeParen {
    fn teleport(&self) -> proc_macro2::TokenStream {
        make_struct!(self, TypeParen, attrs, paren_token, elem)
    }
}

impl Teleporter for TypeNever {
    fn teleport(&self) -> proc_macro2::TokenStream {
        make_struct!(self, TypeNever, attrs, bang_token)
    }
}

impl Teleporter for TypeMacro {
    fn teleport(&self) -> proc_macro2::TokenStream {
        make_struct!(self, TypeMacro, attrs, mac)
    }
}

impl Teleporter for syn::Macro {
    fn teleport(&self) -> proc_macro2::TokenStream {
        make_struct!(self, Macro, path, bang_token, delimiter, tokens)
    }
}

impl Teleporter for MacroDelimiter {
    fn teleport(&self) -> proc_macro2::TokenStream {
        make_enum!(
            self,
            MacroDelimiter,
            Paren(paren),
            Brace(brace),
            Bracket(bracket)
        )
    }
}

impl Teleporter for TypeImplTrait {
    fn teleport(&self) -> proc_macro2::TokenStream {
        make_struct!(self, TypeImplTrait, attrs, impl_token, bounds)
    }
}

impl Teleporter for TypeGroup {
    fn teleport(&self) -> proc_macro2::TokenStream {
        make_struct!(self, TypeGroup, attrs, group_token, elem)
    }
}

impl Teleporter for TypeInfer {
    fn teleport(&self) -> proc_macro2::TokenStream {
        make_struct!(self, TypeInfer, attrs, underscore_token)
    }
}

// We just need this const generics; we may need more as the feature stabilizes.
impl Teleporter for Expr {
    fn teleport(&self) -> proc_macro2::TokenStream {
        make_enum!(
            self,
            Expr,
            Binary(expr_binary),
            Lit(expr_lit),
            Unary(expr_unary),
            Verbatim(token_stream),
            Path(expr_path),
            // The (many) other variants should be unreachable...
        )
    }
}

impl Teleporter for ExprPath {
    fn teleport(&self) -> proc_macro2::TokenStream {
        make_struct!(self, ExprPath, attrs, qself, path)
    }
}

impl Teleporter for ExprLit {
    fn teleport(&self) -> proc_macro2::TokenStream {
        make_struct!(self, ExprLit, attrs, lit)
    }
}

impl Teleporter for ExprUnary {
    fn teleport(&self) -> proc_macro2::TokenStream {
        make_struct!(self, ExprUnary, attrs, op, expr)
    }
}

impl Teleporter for UnOp {
    fn teleport(&self) -> proc_macro2::TokenStream {
        make_enum!(self, UnOp, Deref(star), Not(bang), Neg(minus))
    }
}

impl Teleporter for ExprBinary {
    fn teleport(&self) -> proc_macro2::TokenStream {
        make_struct!(self, ExprBinary, attrs, left, op, right)
    }
}

impl Teleporter for BinOp {
    fn teleport(&self) -> proc_macro2::TokenStream {
        make_enum!(
            self,
            BinOp,
            Add(plus),
            Sub(minus),
            Mul(star),
            Div(slash),
            Rem(percent),
            And(and_and),
            Or(or_or),
            BitXor(caret),
            BitAnd(and),
            BitOr(or),
            Shl(shl),
            Shr(shr),
            Eq(eq_eq),
            Lt(lt),
            Le(le),
            Ne(ne),
            Ge(ge),
            Gt(gt),
            AddAssign(plus_eq),
            SubAssign(minus_eq),
            MulAssign(star_eq),
            DivAssign(slash_eq),
            RemAssign(percent_eq),
            BitXorAssign(caret_eq),
            BitAndAssign(and_eq),
            BitOrAssign(or_eq),
            ShlAssign(shl_eq),
            ShrAssign(shr_eq),
        )
    }
}

impl Teleporter for Lit {
    fn teleport(&self) -> proc_macro2::TokenStream {
        make_enum!(
            self,
            Lit,
            Str(lit_str),
            Int(lit_int),
            Bool(lit_bool),
            // The other variants should be unreachable.
        )
    }
}

impl Teleporter for LitStr {
    fn teleport(&self) -> proc_macro2::TokenStream {
        let value = self.value();
        quote! {
            schema::syn::LitStr::new(#value, schema::proc_macro2::Span::call_site())
        }
    }
}

impl Teleporter for LitInt {
    fn teleport(&self) -> proc_macro2::TokenStream {
        let value = self.base10_digits();
        quote! {
            schema::syn::LitInt::new(#value, schema::proc_macro2::Span::call_site())
        }
    }
}

impl Teleporter for LitBool {
    fn teleport(&self) -> proc_macro2::TokenStream {
        let value = self.value;
        quote! {
            schema::syn::LitBool::new(#value, schema::proc_macro2::Span::call_site())
        }
    }
}

impl Teleporter for Meta {
    fn teleport(&self) -> TokenStream {
        make_enum!(
            self,
            Meta,
            Path(path),
            List(meta_list),
            NameValue(meta_name_value),
        )
    }
}

impl Teleporter for MetaList {
    fn teleport(&self) -> TokenStream {
        make_struct!(self, MetaList, path, delimiter, tokens)
    }
}

impl Teleporter for MetaNameValue {
    fn teleport(&self) -> TokenStream {
        make_struct!(self, MetaNameValue, path, eq_token, value)
    }
}

impl Teleporter for LifetimeParam {
    fn teleport(&self) -> TokenStream {
        make_struct!(self, LifetimeParam, attrs, lifetime, colon_token, bounds)
    }
}

impl Teleporter for AssocType {
    fn teleport(&self) -> TokenStream {
        make_struct!(self, AssocType, ident, generics, eq_token, ty)
    }
}

impl Teleporter for AssocConst {
    fn teleport(&self) -> TokenStream {
        make_struct!(self, AssocConst, ident, generics, eq_token, value)
    }
}

impl Teleporter for PointerMutability {
    fn teleport(&self) -> TokenStream {
        make_enum!(self, PointerMutability, Const(const_), Mut(mut_))
    }
}

impl Teleporter for NamedArg {
    fn teleport(&self) -> TokenStream {
        make_struct!(self, NamedArg, attrs, name, ty)
    }
}

impl Teleporter for TypeFnPtr {
    fn teleport(&self) -> TokenStream {
        make_struct!(
            self,
            TypeFnPtr,
            attrs,
            lifetimes,
            unsafety,
            abi,
            fn_token,
            paren_token,
            inputs,
            variadic,
            output,
        )
    }
}

impl Teleporter for FnPtrVariadic {
    fn teleport(&self) -> TokenStream {
        make_struct!(self, FnPtrVariadic, attrs, name, dots, comma)
    }
}
