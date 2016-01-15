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
    variant_size_differences
)]

extern crate cssparser;
extern crate ego_tree;
extern crate html5ever;
extern crate selectors;
#[macro_use]
extern crate string_cache;
extern crate tendril;

pub use comment::Comment;
pub use doctype::Doctype;
pub use element::Element;
pub use html::Html;
pub use node::Node;
pub use node_ref::NodeRef;
pub use selector::Selector;
pub use text::Text;

pub mod comment;
pub mod doctype;
pub mod element;
pub mod html;
pub mod node;
pub mod node_ref;
pub mod selector;
pub mod text;
