//! Element references.

use std::ops::Deref;

use ego_tree::NodeRef;
use ego_tree::iter::{Traverse, Edge};
use html5ever::serialize::{serialize, SerializeOpts, TraversalScope};

use {Node, Selector};
use node::Element;
use std::collections::HashSet;
use string_cache::atom::Atom;
use std::iter::FromIterator;

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

    fn serialize(&self, traversal_scope: TraversalScope) -> String {
        let opts = SerializeOpts {
            scripting_enabled: false, // It's not clear what this does.
            traversal_scope: traversal_scope,
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
        self.serialize(TraversalScope::ChildrenOnly)
    }

    /// Returns an iterator over descendent text nodes.
    pub fn text(&self) -> Text<'a> {
        Text { inner: self.traverse() }
    }

    /// Returns an iterator over descendent text nodes that is filtered.
    pub fn content(&self,filter: ContentFilter) -> Content<'a> {
        Content { inner: self.traverse(), filter: filter }
    }
}

impl<'a> Deref for ElementRef<'a> {
    type Target = NodeRef<'a, Node>;
    fn deref(&self) -> &NodeRef<'a, Node> { &self.node }
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

impl<'a> Into<String> for Text<'a> {
    fn into(self) -> String {
        return String::from_iter(self);
    }
}

/// Options struct for the text filter.
#[derive(Debug, Clone)]
pub struct ContentFilter {
    system_tags: HashSet<Atom>,
}

impl ContentFilter {
    /// Creates text filter to retrieve user content.
    pub fn new() -> ContentFilter {
        ContentFilter {
            system_tags: HashSet::new(),
        }
    }

    /// Creates text filter to retrieve user content without system tags.
    pub fn text_only() -> ContentFilter {
        let mut filter = ContentFilter::new();
        filter.set_allow_script(false);
        filter.set_allow_stylesheets(false);
        filter.set_allow_frame(false);
        filter.set_allow_object(false);
        filter.set_allow_comment(false);
        filter.set_allow_multimedia(false);
        filter.set_allow_form(false);
        return filter;
    }

    /// Shall use text content of this node?
    pub fn enable_filter(&self,element: &Element) -> bool {
        return self.system_tags.contains(&element.name.local);
    }

    fn set_allow(&mut self,value:bool,tags:Vec<Atom>) -> &mut Self {
        for tag in tags.iter() {
            if value {
                self.system_tags.remove(tag);
            } else {
                self.system_tags.insert(tag.clone());
            }
        }
        return self;
    }


    /// Allow tags: script, noscript.
    pub fn set_allow_script(&mut self,value:bool) -> &mut Self {
        return self.set_allow(
            value, 
            vec![
                atom!("script"),
                atom!("noscript")
            ]
        );
    }

    /// Allow tags: style.
    pub fn set_allow_stylesheets(&mut self,value:bool) -> &mut Self {
        return self.set_allow(
            value, 
            vec![
                atom!("style"),
            ]
        );
    }

    /// Allow tags: frame, frameset, iframe, noframes.
    pub fn set_allow_frame(&mut self,value:bool) -> &mut Self {
        return self.set_allow(
            value, 
            vec![
                atom!("frame"),
                atom!("frameset"),
                atom!("iframe"),
                atom!("noframes"),
            ]
        );
    }

    /// Allow tags: object, embed, applet, param, noembed.
    pub fn set_allow_object(&mut self,value:bool) -> &mut Self {
        return self.set_allow(
            value, 
            vec![
                atom!("object"),
                atom!("embed"),
                atom!("applet"),
                atom!("param"),
                atom!("noembed"),
            ]
        );
    }

    /// Allow tags: comment.
    pub fn set_allow_comment(&mut self,value:bool) -> &mut Self {
        return self.set_allow(
            value, 
            vec![
                atom!("#comment"),
            ]
        );
    }

    /// Allow tags: audio, bgsound, source, track, video, canvas.
    pub fn set_allow_multimedia(&mut self,value:bool) -> &mut Self {
        return self.set_allow(
            value, 
            vec![
                atom!("audio"),
                atom!("bgsound"),
                atom!("source"),
                atom!("track"),
                atom!("video"),
                atom!("canvas"),
            ]
        );
    }

    /// Allow tags: button, fieldset, form, input, datalist, keygen, label, legend, optgroup, option, select, textarea, output, progress, meter.    
    pub fn set_allow_form(&mut self,value:bool) -> &mut Self {
        return self.set_allow(
            value, 
            vec![
                atom!("button"),
                atom!("fieldset"),
                atom!("form"),
                atom!("input"),
                atom!("datalist"),
                atom!("keygen"),
                atom!("label"),
                atom!("legend"),
                atom!("optgroup"),
                atom!("option"),
                atom!("select"),
                atom!("textarea"),
                atom!("output"),
                atom!("progress"),
                atom!("meter"),
            ]
        );
    }
}

impl Default for ContentFilter {
    fn default() -> Self {
        ContentFilter::text_only()
    }
}


/// Iterator over descendent text nodes.
#[derive(Debug, Clone)]
pub struct Content<'a> {
    inner: Traverse<'a, Node>,
    filter: ContentFilter,
}

impl<'a> Iterator for Content<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        let mut filter_enabled = false;
        let mut level = 0usize;
        for edge in &mut self.inner {
            match edge {
                Edge::Open(node) => {                    
                    if let &Node::Element(ref element) = node.value() {
                        filter_enabled = self.filter.enable_filter(&element);
                    }                    
                    if filter_enabled {
                        level = level + 1;                        
                    } else {                        
                        if let &Node::Text(ref text) = node.value() {                            
                            return Some(&*text);
                        }
                    }
                },
                Edge::Close(_) => {                    
                    if filter_enabled {   
                        level = level - 1;                        
                    }
                    if filter_enabled && level == 0 {
                        filter_enabled = false;                        
                    }
                },
            }            
        }
        None
    }
}

impl<'a> Into<String> for Content<'a> {
    fn into(self) -> String {
        return String::from_iter(self);
    }
}


mod element;
mod serializable;
