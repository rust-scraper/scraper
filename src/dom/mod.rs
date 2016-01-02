//! DOM implementation.

use std::borrow::Cow;
use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::{fmt, mem};

use html5ever::tree_builder::QuirksMode;
use string_cache::QualName;
use tendril::StrTendril;
use typed_arena::Arena;

/// Arena-allocated DOM.
pub struct Dom<'a> {
    arena: Arena<TreeNode<'a>>,

    /// Parse errors.
    pub errors: Vec<Cow<'static, str>>,

    /// The document root node.
    pub document: &'a TreeNode<'a>,

    /// The quirks mode.
    pub quirks_mode: QuirksMode,
}

/// A node in the DOM tree.
pub struct TreeNode<'a> {
    /// The DOM node.
    pub node: Node<'a>,

    /// The parent node.
    pub parent: Cell<Option<&'a TreeNode<'a>>>,

    /// The first and last children.
    pub children: Cell<Option<(&'a TreeNode<'a>, &'a TreeNode<'a>)>>,

    /// The next sibling.
    pub next_sibling: Cell<Option<&'a TreeNode<'a>>>,

    /// The previous sibling.
    pub prev_sibling: Cell<Option<&'a TreeNode<'a>>>,
}

/// A DOM node.
#[derive(Debug)]
pub enum Node<'a> {
    /// A document root.
    Document,

    /// A doctype.
    Doctype(Doctype),

    /// A comment.
    Comment(StrTendril),

    /// Text.
    ///
    /// Contained in a `RefCell` so that text can be concatenated together during tree building.
    Text(RefCell<StrTendril>),

    /// An element.
    Element(Element<'a>),
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
pub struct Element<'a> {
    /// Name.
    pub name: QualName,

    /// Attributes.
    ///
    /// Contained in a `RefCell` so that attributes can be added during tree building.
    pub attrs: RefCell<HashMap<QualName, StrTendril>>,

    /// A template element's contents.
    pub template_contents: Option<&'a TreeNode<'a>>,
}

impl<'a> Dom<'a> {
    /// Creates a new DOM.
    pub fn new(quirks_mode: QuirksMode) -> Dom<'a> {
        let mut dom = Dom {
            arena: Arena::new(),
            errors: Vec::new(),
            quirks_mode: quirks_mode,
            document: unsafe { mem::uninitialized() },
        };
        dom.document = dom.new_tree_node(Node::Document);
        dom
    }

    /// Creates a new TreeNode belonging to the DOM.
    pub fn new_tree_node(&self, node: Node<'a>) -> &'a TreeNode<'a> {
        let tree_node = TreeNode {
            node: node,
            parent: Cell::new(None),
            children: Cell::new(None),
            next_sibling: Cell::new(None),
            prev_sibling: Cell::new(None),
        };
        // Convince the compiler that tree_node will live as long as 'a.
        unsafe { mem::transmute(self.arena.alloc(tree_node)) }
    }
}

impl<'a> Default for Dom<'a> {
    fn default() -> Dom<'a> { Dom::new(QuirksMode::NoQuirks) }
}

impl<'a> TreeNode<'a> {
    /// Returns the parent node.
    pub fn parent(&self) -> Option<&'a TreeNode<'a>> {
        self.parent.get()
    }

    /// Returns the first child.
    pub fn first_child(&self) -> Option<&'a TreeNode<'a>> {
        self.children.get().map(|c| c.0)
    }

    /// Returns the last child.
    pub fn last_child(&self) -> Option<&'a TreeNode<'a>> {
        self.children.get().map(|c| c.1)
    }

    /// Returns the next sibling.
    pub fn next_sibling(&self) -> Option<&'a TreeNode<'a>> {
        self.next_sibling.get()
    }

    /// Returns the previous sibling.
    pub fn prev_sibling(&self) -> Option<&'a TreeNode<'a>> {
        self.prev_sibling.get()
    }

    /// Returns an iterator over this node's siblings, starting at this node.
    pub fn iter(&'a self) -> Iter {
        Iter { node: Some(self) }
    }

    /// Returns an iterator over this node's children.
    pub fn children(&'a self) -> Iter {
        if let Some(first) = self.first_child() {
            first.iter()
        } else {
            Iter { node: None }
        }
    }
}

/// `TreeNode` iterator.
#[derive(Debug)]
pub struct Iter<'a> {
    node: Option<&'a TreeNode<'a>>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a TreeNode<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        let node = self.node;
        self.node = self.node.and_then(TreeNode::next_sibling);
        node
    }
}

// Arena does not implement Debug.
impl<'a> fmt::Debug for Dom<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.debug_struct("Dom")
            .field("errors", &self.errors)
            .field("document", &self.document)
            .field("quirks_mode", &self.quirks_mode)
            .finish()
    }
}

// Avoid recursive parts of the structure.
impl<'a> fmt::Debug for TreeNode<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.debug_struct("TreeNode")
            .field("node", &self.node)
            .field("children", &self.children)
            .field("next_sibling", &self.next_sibling)
            .finish()
    }
}
