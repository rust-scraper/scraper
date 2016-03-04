//! HTML nodes.

use std::fmt;
use std::ops::Deref;
use std::collections::{HashSet, HashMap};
use std::collections::{hash_set, hash_map};

use string_cache::{QualName, Atom};
use tendril::StrTendril;

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

/// A doctype.
#[derive(Clone, PartialEq, Eq)]
pub struct Doctype {
    /// The doctype name.
    pub name: StrTendril,

    /// The doctype public ID.
    pub public_id: StrTendril,

    /// The doctype system ID.
    pub system_id: StrTendril,
}

impl Doctype {
    /// Returns the doctype name.
    pub fn name(&self) -> &str {
        self.name.deref()
    }

    /// Returns the doctype public ID.
    pub fn public_id(&self) -> &str {
        self.public_id.deref()
    }

    /// Returns the doctype system ID.
    pub fn system_id(&self) -> &str {
        self.system_id.deref()
    }
}

impl fmt::Debug for Doctype {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            f,
            "<!DOCTYPE {} PUBLIC {:?} {:?}>",
            self.name(),
            self.public_id(),
            self.system_id()
        )
    }
}

/// An HTML comment.
#[derive(Clone, PartialEq, Eq)]
pub struct Comment {
    /// The comment text.
    pub comment: StrTendril,
}

impl Deref for Comment {
    type Target = str;

    fn deref(&self) -> &str {
        self.comment.deref()
    }
}

impl fmt::Debug for Comment {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "<!-- {:?} -->", self.deref())
    }
}

/// HTML text.
#[derive(Clone, PartialEq, Eq)]
pub struct Text {
    /// The text.
    pub text: StrTendril,
}

impl Deref for Text {
    type Target = str;

    fn deref(&self) -> &str {
        self.text.deref()
    }
}

impl fmt::Debug for Text {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{:?}", self.deref())
    }
}

/// An HTML element.
#[derive(Clone, PartialEq, Eq)]
pub struct Element {
    /// The element name.
    pub name: QualName,

    /// The element ID.
    pub id: Option<Atom>,

    /// The element classes.
    pub classes: HashSet<Atom>,

    /// The element attributes.
    pub attrs: HashMap<QualName, StrTendril>,
}

impl Element {
    #[doc(hidden)]
    pub fn new(name: QualName, attrs: HashMap<QualName, StrTendril>) -> Self {
        let id = attrs.get(&qualname!("", "id"))
            .map(Deref::deref)
            .map(Atom::from);

        let classes = attrs.get(&qualname!("", "class"))
            .map(Deref::deref)
            .into_iter()
            .flat_map(str::split_whitespace)
            .map(Atom::from)
            .collect();

        Element {
            name: name,
            id: id,
            classes: classes,
            attrs: attrs,
        }
    }

    /// Returns the element name.
    pub fn name(&self) -> &str {
        self.name.local.deref()
    }

    /// Returns the element ID.
    pub fn id(&self) -> Option<&str> {
        self.id.as_ref().map(Deref::deref)
    }

    /// Returns true if element has the class.
    pub fn has_class(&self, class: &str) -> bool {
        self.classes.contains(&Atom::from(class))
    }

    /// Returns an iterator over the element's classes.
    pub fn classes(&self) -> Classes {
        Classes { inner: self.classes.iter() }
    }

    /// Returns the value of an attribute.
    pub fn attr(&self, attr: &str) -> Option<&str> {
        let qualname = QualName::new(ns!(), Atom::from(attr));
        self.attrs.get(&qualname).map(Deref::deref)
    }

    /// Returns an iterator over the element's attributes.
    pub fn attrs(&self) -> Attrs {
        Attrs { inner: self.attrs.iter() }
    }
}

/// Iterator over classes.
#[allow(missing_debug_implementations)]
#[derive(Clone)]
pub struct Classes<'a> {
    inner: hash_set::Iter<'a, Atom>,
}

impl<'a> Iterator for Classes<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        self.inner.next().map(Deref::deref)
    }
}

/// Iterator over attributes.
#[allow(missing_debug_implementations)]
#[derive(Clone)]
pub struct Attrs<'a> {
    inner: hash_map::Iter<'a, QualName, StrTendril>,
}

impl<'a> Iterator for Attrs<'a> {
    type Item = (&'a str, &'a str);

    fn next(&mut self) -> Option<(&'a str, &'a str)> {
        self.inner.next().map(|(k, v)| (k.local.deref(), v.deref()))
    }
}

impl fmt::Debug for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        try!(write!(f, "<{}", self.name()));
        for (key, value) in self.attrs() {
            try!(write!(f, " {}={:?}", key, value));
        }
        write!(f, ">")
    }
}
