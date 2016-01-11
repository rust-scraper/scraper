//! DOM implementation.

use std::borrow::Cow;
use std::collections::HashMap;

use ego_tree::{Tree, NodeId};
use html5ever::tree_builder::QuirksMode;
use string_cache::QualName;
use tendril::StrTendril;

/// A DOM tree.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dom {
    /// Parse errors.
    pub errors: Vec<Cow<'static, str>>,

    /// The node tree.
    pub tree: Tree<Node>,

    /// The quirks mode.
    pub quirks_mode: QuirksMode,
}

/// A DOM node.
#[derive(Debug, Clone, PartialEq, Eq)]
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Doctype {
    /// Name.
    pub name: StrTendril,

    /// Public ID.
    pub public_id: StrTendril,

    /// System ID.
    pub system_id: StrTendril,
}

/// An element.
#[derive(Debug, Clone, PartialEq, Eq)]
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

impl Node {
    /// Returns true if node is a document root.
    pub fn is_document(&self) -> bool {
        match *self {
            Node::Document => true,
            _ => false,
        }
    }

    /// Returns true if node is a doctype.
    pub fn is_doctype(&self) -> bool {
        match *self {
            Node::Doctype(_) => true,
            _ => false,
        }
    }

    /// Returns true if node is a comment.
    pub fn is_comment(&self) -> bool {
        match *self {
            Node::Comment(_) => true,
            _ => false,
        }
    }

    /// Returns true if node is text.
    pub fn is_text(&self) -> bool {
        match *self {
            Node::Text(_) => true,
            _ => false,
        }
    }

    /// Returns true if node is an element.
    pub fn is_element(&self) -> bool {
        match *self {
            Node::Element(_) => true,
            _ => false,
        }
    }
}

mod tree_sink;
