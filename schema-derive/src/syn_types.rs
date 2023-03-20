use proc_macro2::TokenStream;
use quote::quote;
use syn::*;
use syn::{parse_str, punctuated::Punctuated, Path};

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

    fn name() -> &'static str {
        "syn::Lifetime"
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

impl<T, P> Teleporter for Punctuated<T, P>
where
    T: crate::Teleporter,
    P: crate::Teleporter,
{
    fn teleport(&self) -> proc_macro2::TokenStream {
        let tname = format!("schema::{}", T::name());
        let pname = format!("schema::{}", P::name());
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

    fn name() -> &'static str {
        "syn::Variant"
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
    fn name() -> &'static str {
        "syn::GenericParam"
    }
}
impl Teleporter for TypeParam {
    fn teleport(&self) -> proc_macro2::TokenStream {
        make_struct!(
            self,
            TypeParam,
            attrs,
            ident,
            colon_token,
            bounds,
            eq_token,
            default
        )
    }
}
impl Teleporter for TypeParamBound {
    fn teleport(&self) -> proc_macro2::TokenStream {
        make_enum!(self, TypeParamBound, Trait(trait_bound), Lifetime(lifetime))
    }
    fn name() -> &'static str {
        "syn::TypeParamBound"
    }
}

impl Teleporter for TraitBound {
    fn teleport(&self) -> proc_macro2::TokenStream {
        make_struct!(self, TraitBound, paren_token, modifier, lifetimes, path)
    }
}

impl Teleporter for TraitBoundModifier {
    fn teleport(&self) -> TokenStream {
        make_enum!(self, TraitBoundModifier, Maybe(token), None)
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
            eq_token,
            default,
        )
    }
}

impl Teleporter for Field {
    fn teleport(&self) -> proc_macro2::TokenStream {
        make_struct!(self, Field, attrs, vis, mutability, ident, colon_token, ty)
    }
    fn name() -> &'static str {
        "syn::Field"
    }
}

impl Teleporter for FieldMutability {
    fn teleport(&self) -> TokenStream {
        make_enum!(self, FieldMutability, None)
    }
}

impl Teleporter for Type {
    fn teleport(&self) -> TokenStream {
        make_enum!(
            self,
            Type,
            Array(type_array),
            BareFn(type_bare_fn),
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
    fn name() -> &'static str {
        "syn::Type"
    }
}

impl Teleporter for TypeArray {
    fn teleport(&self) -> TokenStream {
        make_struct!(self, TypeArray, bracket_token, elem, semi_token, len)
    }
}

impl Teleporter for TypeBareFn {
    fn teleport(&self) -> TokenStream {
        make_struct!(
            self,
            TypeBareFn,
            lifetimes,
            unsafety,
            abi,
            fn_token,
            paren_token,
            inputs,
            variadic,
            output
        )
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

impl Teleporter for BareFnArg {
    fn teleport(&self) -> TokenStream {
        make_struct!(self, BareFnArg, attrs, name, ty)
    }
    fn name() -> &'static str {
        "syn::BareFnArg"
    }
}

impl Teleporter for ReturnType {
    fn teleport(&self) -> TokenStream {
        make_enum!(self, ReturnType, Type(rarrow, box_type), Default)
    }
}

impl Teleporter for TypeTuple {
    fn teleport(&self) -> TokenStream {
        make_struct!(self, TypeTuple, paren_token, elems)
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
    fn name() -> &'static str {
        "syn::WherePredicate"
    }
}

impl Teleporter for PredicateType {
    fn teleport(&self) -> TokenStream {
        make_struct!(
            self,
            PredicateType,
            lifetimes,
            bounded_ty,
            colon_token,
            bounds,
        )
    }
}

impl Teleporter for PredicateLifetime {
    fn teleport(&self) -> TokenStream {
        make_struct!(self, PredicateLifetime, lifetime, colon_token, bounds)
    }
}

impl Teleporter for TypeTraitObject {
    fn teleport(&self) -> TokenStream {
        make_struct!(self, TypeTraitObject, dyn_token, bounds)
    }
}

impl Teleporter for TypeReference {
    fn teleport(&self) -> TokenStream {
        make_struct!(self, TypeReference, and_token, lifetime, mutability, elem)
    }
}

impl Teleporter for TypeSlice {
    fn teleport(&self) -> TokenStream {
        make_struct!(self, TypeSlice, bracket_token, elem)
    }
}

impl Teleporter for TypePtr {
    fn teleport(&self) -> proc_macro2::TokenStream {
        make_struct!(self, TypePtr, star_token, const_token, mutability, elem)
    }
}

impl Teleporter for TypePath {
    fn teleport(&self) -> proc_macro2::TokenStream {
        make_struct!(self, TypePath, qself, path)
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
    fn name() -> &'static str {
        "syn::PathSegment"
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
    fn name() -> &'static str {
        "syn::GenericArgument"
    }
}

impl Teleporter for Constraint {
    fn teleport(&self) -> proc_macro2::TokenStream {
        make_struct!(self, Constraint, ident, colon_token, bounds)
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
        make_struct!(self, TypeParen, paren_token, elem)
    }
}

impl Teleporter for TypeNever {
    fn teleport(&self) -> proc_macro2::TokenStream {
        make_struct!(self, TypeNever, bang_token)
    }
}

impl Teleporter for TypeMacro {
    fn teleport(&self) -> proc_macro2::TokenStream {
        make_struct!(self, TypeMacro, mac)
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
        make_struct!(self, TypeImplTrait, impl_token, bounds)
    }
}

impl Teleporter for TypeGroup {
    fn teleport(&self) -> proc_macro2::TokenStream {
        make_struct!(self, TypeGroup, group_token, elem)
    }
}

impl Teleporter for TypeInfer {
    fn teleport(&self) -> proc_macro2::TokenStream {
        make_struct!(self, TypeInfer, underscore_token)
    }
}

// We just need this const generics; we may need more as the feature stabilizes.
impl Teleporter for Expr {
    fn teleport(&self) -> proc_macro2::TokenStream {
        make_enum!(
            self,
            Expr,
            Lit(expr_lit),
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

impl Teleporter for Lit {
    fn teleport(&self) -> proc_macro2::TokenStream {
        make_enum!(
            self,
            Lit,
            Str(lit_str),
            Int(lit_int),
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

impl Teleporter for BareVariadic {
    fn teleport(&self) -> TokenStream {
        make_struct!(self, BareVariadic, attrs, name, dots, comma)
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
