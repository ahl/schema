//! This crate includes the [`Schema`] trait and an associated derive macro. The
//! [`Schema::schema`] method produces a [`syn::DeriveInput`] struct (which this
//! crate re-exports) that represents the schema of the type on which the method is
//! invoked.
//!
//! Typically one operates with `syn` in proc macro context, parsing a
//! [`proc_macro::TokenStream`] into a [`syn::DeriveInput`]. Operating in proc
//! macro context can be tricky. It may be desireable to do early development,
//! testing, or non-performance critical work in program context instead. For
//! example, if one were to encode the schema for a type in a standard such as JSON
//! Schema, it might be simpler, more expedient, and sufficiently efficient to do
//! so in program context rather than building the full derive macro.
//!
//! This crate is something of an experiment and certainly a work in progress.
//! Feedback, suggestions for improvements, and--especially--PRs are very welcome.

// The generated impl uses these crates, but we treat this as an implementation
// detail.
#[doc(hidden)]
pub use proc_macro2;
#[doc(hidden)]
pub use quote;
#[doc(hidden)]
pub use syn;

extern crate schema_derive;
pub use schema_derive::Schema;
pub use syn::DeriveInput;

/// Produces a schema for the type expressed as a [DeriveInput], re-exported from
/// the syn crate.
pub trait Schema {
    fn schema() -> DeriveInput;
}
