//! hope.

#![warn(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results,
    variant_size_differences
)]

#[macro_use]
extern crate string_cache;
extern crate tendril;

pub mod doctype;
pub mod comment;
pub mod element;
