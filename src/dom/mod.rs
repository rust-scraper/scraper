//! DOM implementation.

use std::borrow::Cow;
use std::collections::HashMap;

use ego_tree::{Tree, NodeId};
use html5ever::tree_builder::QuirksMode;
use string_cache::QualName;
use tendril::StrTendril;

/// A DOM tree.
#[derive(Debug)]
pub struct Dom {
    /// Parse errors.
    pub errors: Vec<Cow<'static, str>>,

    /// The node tree.
    pub tree: Tree<Node>,

    /// The quirks mode.
    pub quirks_mode: QuirksMode,
}

/// A DOM node.
#[derive(Debug)]
pub enum Node {
    /// A document root.
    Document,

    /// A doctype.
    Doctype(Doctype),

    /// A comment.
    Comment(StrTendril),

    /// Text.
    Text(StrTendril),

    /// An element.
    Element(Element),
}

/// A doctype.
#[derive(Debug)]
pub struct Doctype {
    /// Name.
    pub name: StrTendril,

    /// Public ID.
    pub public_id: StrTendril,

    /// System ID.
    pub system_id: StrTendril,
}

/// An element.
#[derive(Debug)]
pub struct Element {
    /// Name.
    pub name: QualName,

    /// Attributes.
    pub attrs: HashMap<QualName, StrTendril>,

    /// The tree node ID of a template element's contents.
    pub template_contents_id: Option<NodeId<Node>>,
}

impl Dom {
    /// Creates a new DOM.
    pub fn new(quirks_mode: QuirksMode) -> Self {
        Dom {
            errors: Vec::new(),
            tree: Tree::new(Node::Document),
            quirks_mode: quirks_mode,
        }
    }
}

impl Default for Dom {
    fn default() -> Self { Dom::new(QuirksMode::NoQuirks) }
}

mod tree_sink;
