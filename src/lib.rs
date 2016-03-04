//! HTML parsing and querying with CSS selectors.
//!
//! `scraper` is on [Crates.io][crate] and [GitHub][github].
//!
//! [crate]: https://crates.io/crates/scraper
//! [github]: https://github.com/programble/scraper
//!
//! Scraper provides an interface to Servo's `html5ever` and `selectors` crates, for browser-grade
//! parsing and querying.
//!
//! # Examples
//!
//! ## Parsing a document
//!
//! ```
//! use scraper::Html;
//!
//! let html = r#"
//!     <!DOCTYPE html>
//!     <meta charset="utf-8">
//!     <title>Hello, world!</title>
//!     <h1>Hello, <i>world!</i></h1>
//! "#;
//!
//! let document = Html::parse_document(html);
//! assert!(document.errors.is_empty());
//! ```
//!
//! ## Parsing a fragment
//!
//! ```
//! # use scraper::Html;
//! let fragment = Html::parse_fragment("<h1>Hello, world!</h1>");
//! assert!(fragment.errors.is_empty());
//! ```
//!
//! ## Parsing a selector
//!
//! ```
//! use scraper::Selector;
//! let selector = Selector::parse("h1").unwrap();
//! ```
//!
//! ## Selecting elements
//!
//! ```
//! # use scraper::{Html, Selector};
//! # let html = r#"
//! #     <!DOCTYPE html>
//! #     <meta charset="utf-8">
//! #     <title>Hello, world!</title>
//! #     <h1>Hello, <i>world!</i></h1>
//! # "#;
//! # let document = Html::parse_document(html);
//! # let selector = Selector::parse("h1").unwrap();
//! for node in document.select(&selector) {
//!     assert_eq!("h1", node.value().as_element().unwrap().name());
//! }
//! ```
//!
//! ## Selecting child elements
//!
//! ```
//! # use scraper::{Html, Selector};
//! # let document = Html::parse_document("<h1>Hello, <i>world!</i></h1>");
//! # let selector = Selector::parse("h1").unwrap();
//! let h1 = document.select(&selector).next().unwrap();
//! for node in h1.select(&Selector::parse("i").unwrap()) {
//!     assert_eq!("i", node.value().as_element().unwrap().name());
//! }
//! ```
//!
//! ## Accessing element attributes
//!
//! ```
//! # use scraper::{Html, Selector};
//! let fragment = Html::parse_fragment(r#"<input type="hidden" name="foo" value="bar">"#);
//! let selector = Selector::parse(r#"input[name="foo"]"#).unwrap();
//!
//! let input = fragment.select(&selector).next().unwrap();
//! let value = input.value()
//!     .as_element()
//!     .unwrap()
//!     .attr("value")
//!     .unwrap();
//!
//! assert_eq!("bar", value);
//! ```
//!
//! ## Accessing text
//!
//! ```
//! # use scraper::{Html, Selector};
//! let fragment = Html::parse_fragment("<h1>Hello, <i>world!</i></h1>");
//! let selector = Selector::parse("h1").unwrap();
//!
//! let h1 = fragment.select(&selector).next().unwrap();
//! let text = h1.text().collect::<Vec<_>>();
//!
//! assert_eq!(vec!["Hello, ", "world!"], text);
//! ```

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

pub use html::Html;
pub use node::Node;
pub use node_ref::NodeRef;
pub use selector::Selector;

pub mod html;
pub mod node;
pub mod node_ref;
pub mod selector;
