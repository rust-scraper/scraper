#![doc = include_str!("../README.md")]
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

//! # The core types
//!
//! Parsing an input produces an [`Html`] document that owns a tree of
//! [`Node`]s. Every node is one variant of the [`Node`] enum, for example
//! [`Node::Text`] for text and [`Node::Element`] for an element. The data that
//! belongs to an element node, its name and its attributes, is held in a
//! [`node::Element`](crate::node::Element).
//!
//! Running a [`Selector`] over a document does not hand back bare [`Node`]s.
//! It yields [`ElementRef`]s, each one a handle to an element node that also
//! knows where it sits in the tree. From an [`ElementRef`] you can reach the
//! element data with [`ElementRef::value`], read the text under it with
//! [`ElementRef::text`], or select further into its descendants. Because an
//! [`ElementRef`] also dereferences to an `ego_tree::NodeRef`, the tree
//! navigation methods (parent, children, siblings) are available on it too.
//!
//! The re-exported [`Element`] trait is a different thing from
//! [`node::Element`](crate::node::Element). The trait comes from the
//! `selectors` crate and is what lets an [`ElementRef`] be matched against a
//! CSS selector. Most code never needs to name it directly.
//!
//! ```
//! use scraper::{Html, Selector};
//!
//! let document = Html::parse_fragment(r#"<ul><li id="a">one</li><li>two</li></ul>"#);
//! let selector = Selector::parse("li").unwrap();
//!
//! for element in document.select(&selector) {
//!     // The element data: its tag name and attributes.
//!     let name = element.value().name();
//!     let id = element.value().id();
//!     // The text nodes below this element, concatenated.
//!     let text = element.text().collect::<String>();
//!     println!("{name} id={id:?} text={text:?}");
//! }
//! ```

#[macro_use]
extern crate html5ever;

pub use crate::element_ref::ElementRef;
pub use crate::html::{Html, HtmlTreeSink};
pub use crate::node::Node;
pub use crate::selector::Selector;

pub use selectors::{Element, attr::CaseSensitivity};

pub mod element_ref;
pub mod error;
pub mod html;
pub mod node;
pub mod selectable;
pub mod selector;

#[cfg(feature = "atomic")]
pub(crate) mod tendril_util {
    use html5ever::tendril;
    /// Atomic equivalent to the default `StrTendril` type.
    pub type StrTendril = tendril::Tendril<tendril::fmt::UTF8, tendril::Atomic>;

    /// Convert a standard tendril into an atomic one.
    pub fn make(s: tendril::StrTendril) -> StrTendril {
        s.into_send().into()
    }
}

#[cfg(not(feature = "atomic"))]
pub(crate) mod tendril_util {
    use html5ever::tendril;
    /// Primary string tendril type.
    pub type StrTendril = tendril::StrTendril;

    /// Return unaltered.
    pub fn make(s: StrTendril) -> StrTendril {
        s
    }
}

pub use tendril_util::StrTendril;

#[cfg(test)]
mod test;
