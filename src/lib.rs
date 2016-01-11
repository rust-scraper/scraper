//! # Scraper
//!
//! Dumpster diving for that precious web content.

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

extern crate ego_tree;
extern crate html5ever;
#[macro_use] extern crate string_cache;
extern crate tendril;

use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt;

use ego_tree::Tree;
use html5ever::driver;
use html5ever::tree_builder::QuirksMode;
use string_cache::QualName;
use tendril::StrTendril;

/// A parsed HTML tree.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Html {
    /// Parse errors.
    pub errors: Vec<Cow<'static, str>>,

    /// The quirks mode.
    pub quirks_mode: QuirksMode,

    /// The node tree.
    pub tree: Tree<HtmlNode>,
}

impl Html {
    /// Creates an empty HTML tree.
    pub fn new(quirks_mode: QuirksMode) -> Self {
        Html {
            errors: Vec::new(),
            quirks_mode: quirks_mode,
            tree: Tree::new(HtmlNode::Document),
        }
    }

    /// Parses an HTML document.
    pub fn parse(s: &str) -> Self {
        driver::parse_to(
            Self::default(),
            driver::one_input(StrTendril::from_slice(s)),
            Default::default()
        )
    }
}

impl Default for Html {
    fn default() -> Self { Html::new(QuirksMode::NoQuirks) }
}

/// An HTML node.
#[derive(Clone, PartialEq, Eq)]
pub enum HtmlNode {
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

impl HtmlNode {
    /// Returns true if node is a document root.
    pub fn is_document(&self) -> bool {
        match *self { HtmlNode::Document => true, _ => false }
    }

    /// Returns true if node is a doctype.
    pub fn is_doctype(&self) -> bool {
        match *self { HtmlNode::Doctype(_) => true, _ => false }
    }

    /// Returns true if node is a comment.
    pub fn is_comment(&self) -> bool {
        match *self { HtmlNode::Comment(_) => true, _ => false }
    }

    /// Returns true if node is text.
    pub fn is_text(&self) -> bool {
        match *self { HtmlNode::Text(_) => true, _ => false }
    }

    /// Returns true if node is an element.
    pub fn is_element(&self) -> bool {
        match *self { HtmlNode::Element(_) => true, _ => false }
    }

    /// Returns self as a `Doctype`, if possible.
    pub fn as_doctype(&self) -> Option<&Doctype> {
        match *self {
            HtmlNode::Doctype(ref doctype) => Some(doctype),
            _ => None,
        }
    }

    /// Returns self as a comment `StrTendril`, if possible.
    pub fn as_comment(&self) -> Option<&StrTendril> {
        match *self {
            HtmlNode::Comment(ref comment) => Some(comment),
            _ => None,
        }
    }

    /// Returns self as a text `StrTendril`, if possible.
    pub fn as_text(&self) -> Option<&StrTendril> {
        match *self {
            HtmlNode::Text(ref text) => Some(text),
            _ => None,
        }
    }

    /// Returns self as an `Element`, if possible.
    pub fn as_element(&self) -> Option<&Element> {
        match *self {
            HtmlNode::Element(ref element) => Some(element),
            _ => None,
        }
    }

    /// Returns self as a mutable `Doctype`, if possible.
    pub fn as_doctype_mut(&mut self) -> Option<&mut Doctype> {
        match *self {
            HtmlNode::Doctype(ref mut doctype) => Some(doctype),
            _ => None,
        }
    }

    /// Returns self as a mutable comment `StrTendril`, if possible.
    pub fn as_comment_mut(&mut self) -> Option<&mut StrTendril> {
        match *self {
            HtmlNode::Comment(ref mut comment) => Some(comment),
            _ => None,
        }
    }

    /// Returns self as a mutable text `StrTendril`, if possible.
    pub fn as_text_mut(&mut self) -> Option<&mut StrTendril> {
        match *self {
            HtmlNode::Text(ref mut text) => Some(text),
            _ => None,
        }
    }

    /// Returns self as a mutable `Element`, if possible.
    pub fn as_element_mut(&mut self) -> Option<&mut Element> {
        match *self {
            HtmlNode::Element(ref mut element) => Some(element),
            _ => None,
        }
    }
}

impl fmt::Debug for HtmlNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            HtmlNode::Document => write!(f, "Document"),
            HtmlNode::Doctype(ref doctype) => write!(f, "Doctype({:?})", doctype),
            HtmlNode::Comment(ref comment) => write!(f, "Comment({:?})", &comment[..]),
            HtmlNode::Text(ref text) => write!(f, "Text({:?})", &text[..]),
            HtmlNode::Element(ref element) => write!(f, "Element({:?})", element),
        }
    }
}

/// A doctype.
#[derive(Clone, PartialEq, Eq)]
pub struct Doctype {
    /// Name.
    pub name: StrTendril,

    /// Public ID.
    pub public_id: StrTendril,

    /// System ID.
    pub system_id: StrTendril,
}

impl fmt::Debug for Doctype {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            f,
            "<!DOCTYPE {} PUBLIC {:?} {:?}>",
            self.name,
            &self.public_id[..],
            &self.system_id[..]
        )
    }
}

/// An HTML element.
#[derive(Clone, PartialEq, Eq)]
pub struct Element {
    /// Name.
    pub name: QualName,

    /// Attributes.
    pub attrs: HashMap<QualName, StrTendril>,
}

impl fmt::Debug for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        try!(write!(f, "<{}", self.name.local));
        for (key, value) in &self.attrs {
            try!(write!(f, " {}={:?}", key.local, &value[..]));
        }
        write!(f, ">")
    }
}

mod tree_sink;
