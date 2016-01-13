//! HTML elements.

use std::collections::{HashSet, HashMap};
use std::collections::{hash_set, hash_map};
use std::fmt;
use std::ops::Deref;

use string_cache::{QualName, Atom};
use tendril::StrTendril;

/// An HTML element.
#[derive(Clone, PartialEq, Eq)]
pub struct Element {
    name: QualName,
    id: Atom,
    classes: HashSet<Atom>,
    attrs: HashMap<QualName, StrTendril>,
}

impl Element {
    /// Returns the element name.
    pub fn name(&self) -> &str {
        self.name.local.deref()
    }

    /// Returns the element ID.
    pub fn id(&self) -> &str {
        self.id.deref()
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
