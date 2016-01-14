//! HTML documents and fragments.

use std::borrow::Cow;

use ego_tree::Tree;
use html5ever::tree_builder::QuirksMode;

use node::Node;

/// An HTML tree.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Html {
    /// Parse errors.
    pub errors: Vec<Cow<'static, str>>,

    /// The quirks mode.
    pub quirks_mode: QuirksMode,

    /// The node tree.
    pub tree: Tree<Node>,
}

impl Html {
    /// Creates an empty HTML document.
    pub fn new_document() -> Self {
        Html {
            errors: Vec::new(),
            quirks_mode: QuirksMode::NoQuirks,
            tree: Tree::new(Node::Document),
        }
    }

    /// Creates an empty HTML fragment.
    pub fn new_fragment() -> Self {
        Html {
            errors: Vec::new(),
            quirks_mode: QuirksMode::NoQuirks,
            tree: Tree::new(Node::Fragment),
        }
    }

    /// Parses a string of HTML as a document.
    pub fn parse_document(document: &str) -> Self {
        unimplemented!()
    }

    /// Parses a string of HTML as a fragment.
    pub fn parse_fragment(fragment: &str) -> Self {
        unimplemented!()
    }
}

mod tree_sink;
