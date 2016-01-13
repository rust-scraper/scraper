//! HTML.

use doctype::Doctype;
use comment::Comment;
use text::Text;
use element::Element;

/// An HTML node.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Html {
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

impl Html {
    /// Returns true if node is the document root.
    pub fn is_document(&self) -> bool {
        match *self { Html::Document => true, _ => false }
    }

    /// Returns true if node is the fragment root.
    pub fn is_fragment(&self) -> bool {
        match *self { Html::Fragment => true, _ => false }
    }

    /// Returns true if node is a doctype.
    pub fn is_doctype(&self) -> bool {
        match *self { Html::Doctype(_) => true, _ => false }
    }

    /// Returns true if node is a comment.
    pub fn is_comment(&self) -> bool {
        match *self { Html::Comment(_) => true, _ => false }
    }

    /// Returns true if node is text.
    pub fn is_text(&self) -> bool {
        match *self { Html::Text(_) => true, _ => false }
    }

    /// Returns true if node is an element.
    pub fn is_element(&self) -> bool {
        match *self { Html::Element(_) => true, _ => false }
    }

    /// Returns self as a doctype.
    pub fn as_doctype(&self) -> Option<&Doctype> {
        match *self { Html::Doctype(ref d) => Some(d), _ => None }
    }

    /// Returns self as a comment.
    pub fn as_comment(&self) -> Option<&Comment> {
        match *self { Html::Comment(ref c) => Some(c), _ => None }
    }

    /// Returns self as text.
    pub fn as_text(&self) -> Option<&Text> {
        match *self { Html::Text(ref t) => Some(t), _ => None }
    }

    /// Returns self as an element.
    pub fn as_element(&self) -> Option<&Element> {
        match *self { Html::Element(ref e) => Some(e), _ => None }
    }
}
