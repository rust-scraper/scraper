//! Element references.

use std::ops::Deref;

use ego_tree::iter::{Edge, Traverse};
use ego_tree::NodeRef;
use html5ever::serialize::{serialize, SerializeOpts, TraversalScope};

use node::Element;
use selectors::parser::SelectorParseErrorKind;
use {Node, Selector};

/// Wrapper around a reference to an element node.
///
/// This wrapper implements the `Element` trait from the `selectors` crate, which allows it to be
/// matched against CSS selectors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ElementRef<'a> {
    node: NodeRef<'a, Node>,
}

impl<'a> ElementRef<'a> {
    fn new(node: NodeRef<'a, Node>) -> Self {
        ElementRef { node: node }
    }

    /// Wraps a `NodeRef` only if it references a `Node::Element`.
    pub fn wrap(node: NodeRef<'a, Node>) -> Option<Self> {
        if node.value().is_element() {
            Some(ElementRef::new(node))
        } else {
            None
        }
    }

    /// Returns the `Element` referenced by `self`.
    pub fn value(&self) -> &'a Element {
        self.node.value().as_element().unwrap()
    }

    /// Returns an iterator over descendent elements matching a selector.
    pub fn select<'b>(&self, selector: &'b Selector) -> Select<'a, 'b> {
        let mut inner = self.traverse();
        inner.next(); // Skip Edge::Open(self).

        Select {
            inner: inner,
            selector: selector,
        }
    }

    /// Returns an iterator over descendent elements matching a selector.
    pub fn try_select<'s>(
        &self,
        selector: &'s str,
    ) -> Result<SelectOwned<'_>, cssparser::ParseError<'s, SelectorParseErrorKind<'s>>> {
        let mut inner = self.traverse();
        inner.next(); // Skip Edge::Open(self).
        let selector = Selector::parse(selector)?;

        Ok(SelectOwned { inner, selector })
    }

    fn serialize(&self, traversal_scope: TraversalScope) -> String {
        let opts = SerializeOpts {
            scripting_enabled: false, // It's not clear what this does.
            traversal_scope: traversal_scope,
            create_missing_parent: false,
        };
        let mut buf = Vec::new();
        serialize(&mut buf, self, opts).unwrap();
        String::from_utf8(buf).unwrap()
    }

    /// Returns the HTML of this element.
    pub fn html(&self) -> String {
        self.serialize(TraversalScope::IncludeNode)
    }

    /// Returns the inner HTML of this element.
    pub fn inner_html(&self) -> String {
        self.serialize(TraversalScope::ChildrenOnly(None))
    }

    /// Returns an iterator over descendent text nodes.
    pub fn text(&self) -> Text<'a> {
        Text {
            inner: self.traverse(),
        }
    }
}

impl<'a> Deref for ElementRef<'a> {
    type Target = NodeRef<'a, Node>;
    fn deref(&self) -> &NodeRef<'a, Node> {
        &self.node
    }
}

/// Iterator over descendent elements matching a selector.
#[derive(Debug, Clone)]
pub struct Select<'a, 'b> {
    inner: Traverse<'a, Node>,
    selector: &'b Selector,
}

impl<'a, 'b> Iterator for Select<'a, 'b> {
    type Item = ElementRef<'a>;

    fn next(&mut self) -> Option<ElementRef<'a>> {
        for edge in &mut self.inner {
            if let Edge::Open(node) = edge {
                if let Some(element) = ElementRef::wrap(node) {
                    if self.selector.matches(&element) {
                        return Some(element);
                    }
                }
            }
        }
        None
    }
}

/// Iterator over descendent elements matching a selector.
#[derive(Debug, Clone)]
pub struct SelectOwned<'a> {
    inner: Traverse<'a, Node>,
    selector: Selector,
}

impl<'a> Iterator for SelectOwned<'a> {
    type Item = ElementRef<'a>;

    fn next(&mut self) -> Option<ElementRef<'a>> {
        for edge in &mut self.inner {
            if let Edge::Open(node) = edge {
                if let Some(element) = ElementRef::wrap(node) {
                    if self.selector.matches(&element) {
                        return Some(element);
                    }
                }
            }
        }
        None
    }
}

/// Iterator over descendent text nodes.
#[derive(Debug, Clone)]
pub struct Text<'a> {
    inner: Traverse<'a, Node>,
}

impl<'a> Iterator for Text<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        for edge in &mut self.inner {
            if let Edge::Open(node) = edge {
                if let &Node::Text(ref text) = node.value() {
                    return Some(&*text);
                }
            }
        }
        None
    }
}

mod element;
mod serializable;
