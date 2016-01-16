//! HTML nodes.

use std::fmt;

use {Doctype, Comment, Text, Element};

/// An HTML node.
#[derive(Clone, PartialEq, Eq)]
pub enum Node {
    /// The document root.
    Document,

    /// The fragment root.
    Fragment,

    /// A doctype.
    Doctype(Doctype),

    /// A comment.
    Comment(Comment),

    /// Text.
    Text(Text),

    /// An element.
    Element(Element),
}

impl Node {
    /// Returns true if node is the document root.
    pub fn is_document(&self) -> bool {
        match *self { Node::Document => true, _ => false }
    }

    /// Returns true if node is the fragment root.
    pub fn is_fragment(&self) -> bool {
        match *self { Node::Fragment => true, _ => false }
    }

    /// Returns true if node is a doctype.
    pub fn is_doctype(&self) -> bool {
        match *self { Node::Doctype(_) => true, _ => false }
    }

    /// Returns true if node is a comment.
    pub fn is_comment(&self) -> bool {
        match *self { Node::Comment(_) => true, _ => false }
    }

    /// Returns true if node is text.
    pub fn is_text(&self) -> bool {
        match *self { Node::Text(_) => true, _ => false }
    }

    /// Returns true if node is an element.
    pub fn is_element(&self) -> bool {
        match *self { Node::Element(_) => true, _ => false }
    }

    /// Returns self as a doctype.
    pub fn as_doctype(&self) -> Option<&Doctype> {
        match *self { Node::Doctype(ref d) => Some(d), _ => None }
    }

    /// Returns self as a comment.
    pub fn as_comment(&self) -> Option<&Comment> {
        match *self { Node::Comment(ref c) => Some(c), _ => None }
    }

    /// Returns self as text.
    pub fn as_text(&self) -> Option<&Text> {
        match *self { Node::Text(ref t) => Some(t), _ => None }
    }

    /// Returns self as an element.
    pub fn as_element(&self) -> Option<&Element> {
        match *self { Node::Element(ref e) => Some(e), _ => None }
    }
}

// Always use one line.
impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            Node::Document => write!(f, "Document"),
            Node::Fragment => write!(f, "Fragment"),
            Node::Doctype(ref d) => write!(f, "Doctype({:?})", d),
            Node::Comment(ref c) => write!(f, "Comment({:?})", c),
            Node::Text(ref t) => write!(f, "Text({:?})", t),
            Node::Element(ref e) => write!(f, "Element({:?})", e),
        }
    }
}
