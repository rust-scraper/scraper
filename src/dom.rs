//! DOM implementation.

use std::borrow::Cow;
use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::fmt;
use std::mem;
use std::ops::Deref;

use html5ever::Attribute;
use html5ever::driver::ParseResult;
use html5ever::tree_builder::{TreeSink, QuirksMode, NodeOrText};
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
#[derive(Debug)]
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
    /// The document itself.
    Document,

    /// A doctype.
    Doctype(Doctype),

    /// A comment.
    Comment(StrTendril),

    /// Text.
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
    pub attrs: RefCell<HashMap<QualName, StrTendril>>,
    /// A script element's "already started" flag.
    pub script_already_started: Cell<Option<bool>>,
    /// A template element's contents.
    pub template_contents: Option<&'a TreeNode<'a>>,
}

/// A reference to a `TreeNode`.
#[derive(Debug, Clone)]
pub struct Handle<'a>(&'a TreeNode<'a>);
impl<'a> Deref for Handle<'a> {
    type Target = TreeNode<'a>;
    fn deref(&self) -> &TreeNode<'a> { self.0 }
}

impl<'a> Dom<'a> {
    /// Creates a TreeNode in the arena.
    fn create_tree_node(&self, node: Node<'a>) -> &'a TreeNode<'a> {
        let node = TreeNode {
            node: node,
            parent: Cell::new(None),
            children: Cell::new(None),
            next_sibling: Cell::new(None),
            prev_sibling: Cell::new(None),
        };
        // Convince the compiler that node will live as long as 'a.
        unsafe { mem::transmute(self.arena.alloc(node)) }
    }
}

impl<'a> fmt::Debug for Dom<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.debug_struct("Dom")
            .field("errors", &self.errors)
            .field("document", &self.document)
            .field("quirks_mode", &self.quirks_mode)
            .finish()
    }
}

impl<'a> TreeNode<'a> {
    fn append_child(&'a self, child: &'a TreeNode<'a>) {
        if let Some((first, last)) = self.children.get() {
            last.next_sibling.set(Some(child));
            child.prev_sibling.set(Some(last));
            self.children.set(Some((first, child)));
        } else {
            self.children.set(Some((child, child)));
        }
        child.parent.set(Some(self));
    }

    fn insert_before(&'a self, sibling: &'a TreeNode<'a>) {
        let parent = self.parent.get().unwrap();

        if let Some(prev) = self.prev_sibling.get() {
            sibling.prev_sibling.set(Some(prev));
            sibling.next_sibling.set(Some(self));
            prev.next_sibling.set(Some(sibling));
            self.prev_sibling.set(Some(sibling));
        } else {
            sibling.next_sibling.set(Some(self));
            self.prev_sibling.set(Some(sibling));
            let last = parent.children.get().unwrap().1;
            parent.children.set(Some((sibling, last)));
        }

        sibling.parent.set(Some(parent));
    }
}

#[allow(trivial_casts)]
fn same_node<'a>(a: &TreeNode<'a>, b: &TreeNode<'a>) -> bool {
    a as *const _ == b as *const _
}

impl<'a> TreeSink for Dom<'a> {
    type Handle = Handle<'a>;

    fn parse_error(&mut self, msg: Cow<'static, str>) {
        self.errors.push(msg);
    }

    fn get_document(&mut self) -> Handle<'a> {
        Handle(self.document)
    }

    fn get_template_contents(&self, target: Self::Handle) -> Self::Handle {
        let Handle(node) = target;
        if let Node::Element(ref element) = node.node {
            Handle(element.template_contents.unwrap())
        } else {
            panic!("not an element")
        }
    }

    fn same_node(&self, x: Handle<'a>, y: Handle<'a>) -> bool {
        same_node(x.0, y.0)
    }

    fn elem_name(&self, target: Handle<'a>) -> QualName {
        let Handle(node) = target;
        if let Node::Element(ref element) = node.node {
            element.name.clone()
        } else {
            panic!("not an element")
        }
    }

    fn set_quirks_mode(&mut self, mode: QuirksMode) {
        self.quirks_mode = mode;
    }

    fn create_element(&mut self, name: QualName, attrs: Vec<Attribute>) -> Handle<'a> {
        let attrs = attrs.into_iter()
            .map(|a| (a.name, a.value))
            .collect();

        if name == qualname!(html, "template") {
            let contents = self.create_tree_node(Node::Document);
            let element = Element {
                name: name,
                attrs: RefCell::new(attrs),
                script_already_started: Cell::new(None),
                template_contents: Some(contents),
            };
            Handle(self.create_tree_node(Node::Element(element)))
        } else {
            let element = Element {
                name: name,
                attrs: RefCell::new(attrs),
                script_already_started: Cell::new(None),
                template_contents: None,
            };
            if element.name == qualname!(html, "script") {
                element.script_already_started.set(Some(false));
            }
            Handle(self.create_tree_node(Node::Element(element)))
        }
    }

    fn create_comment(&mut self, text: StrTendril) -> Handle<'a> {
        Handle(self.create_tree_node(Node::Comment(text)))
    }

    fn append(&mut self, parent: Handle<'a>, child: NodeOrText<Handle<'a>>) {
        let Handle(parent) = parent;
        match child {
            NodeOrText::AppendNode(Handle(node)) => {
                parent.append_child(node);
            },

            NodeOrText::AppendText(text) => {
                let siblings = parent.children.get();

                if let Some((_, &TreeNode { node: Node::Text(ref tendril), .. })) = siblings {
                    tendril.borrow_mut().push_tendril(&text);
                } else {
                    let node = Node::Text(RefCell::new(text));
                    parent.append_child(self.create_tree_node(node));
                }
            },
        }
    }

    fn append_before_sibling(
        &mut self,
        sibling: Handle<'a>,
        new_node: NodeOrText<Handle<'a>>
    ) -> Result<(), NodeOrText<Handle<'a>>> {
        let Handle(sibling) = sibling;
        if sibling.parent.get().is_none() {
            return Err(new_node)
        }

        match new_node {
            NodeOrText::AppendNode(Handle(node)) => {
                self.remove_from_parent(Handle(node));
                sibling.insert_before(node);
            },

            NodeOrText::AppendText(text) => {
                let prev_sibling = sibling.prev_sibling.get();
                if let Some(&TreeNode { node: Node::Text(ref tendril), .. }) = prev_sibling {
                    tendril.borrow_mut().push_tendril(&text);
                } else {
                    let node = Node::Text(RefCell::new(text));
                    sibling.insert_before(self.create_tree_node(node));
                }
            },
        }

        Ok(())
    }

    fn append_doctype_to_document(&mut self, name: StrTendril, public_id: StrTendril, system_id: StrTendril) {
        let doctype = Doctype {
            name: name,
            public_id: public_id,
            system_id: system_id,
        };
        self.document.append_child(self.create_tree_node(Node::Doctype(doctype)));
    }

    fn add_attrs_if_missing(&mut self, target: Handle<'a>, attrs: Vec<Attribute>) {
        let Handle(node) = target;
        if let Node::Element(ref element) = node.node {
            let mut elem_attrs = element.attrs.borrow_mut();
            for attr in attrs {
                if !elem_attrs.contains_key(&attr.name) {
                    let _ = elem_attrs.insert(attr.name, attr.value);
                }
            }
        } else {
            panic!("not an element");
        }
    }

    fn remove_from_parent(&mut self, target: Handle<'a>) {
        let Handle(node) = target;
        let parent = match node.parent.get() {
            Some(p) => p,
            None => return,
        };
        let prev_sibling = node.prev_sibling.get();
        let next_sibling = node.next_sibling.get();

        // Update sibling refs.
        if let Some(sibling) = prev_sibling {
            sibling.next_sibling.set(node.next_sibling.get());
        }
        if let Some(sibling) = next_sibling {
            sibling.prev_sibling.set(node.prev_sibling.get());
        }

        // Update parent refs.
        if prev_sibling.is_none() && next_sibling.is_none() {
            parent.children.set(None);
        } else {
            let mut parent_children = parent.children.get().unwrap();
            if same_node(parent_children.0, node) {
                parent_children.0 = next_sibling.unwrap();
            }
            if same_node(parent_children.1, node) {
                parent_children.1 = prev_sibling.unwrap();
            }
            parent.children.set(Some(parent_children));
        }

        // Orphan node.
        node.parent.set(None);
        node.next_sibling.set(None);
        node.prev_sibling.set(None);
    }

    fn reparent_children(&mut self, node: Handle<'a>, new_parent: Handle<'a>) {
        let Handle(old_parent) = node;
        let Handle(new_parent) = new_parent;
        let children = match old_parent.children.get() {
            Some(c) => c,
            None => return,
        };

        // Orphan children.
        old_parent.children.set(None);

        // Adopt children.
        let mut child = Some(children.0);
        while let Some(node) = child {
            node.parent.set(Some(new_parent));
            child = node.next_sibling.get();
        }

        // Append children to their new siblings.
        if let Some((first, last)) = new_parent.children.get() {
            last.next_sibling.set(Some(children.0));
            children.0.prev_sibling.set(Some(last));
            new_parent.children.set(Some((first, children.1)));
        } else {
            new_parent.children.set(Some(children));
        }
    }

    fn mark_script_already_started(&mut self, node: Handle<'a>) {
        let Handle(node) = node;
        if let Node::Element(ref element) = node.node {
            if element.script_already_started.get().is_none() {
                panic!("not a script element");
            }
            element.script_already_started.set(Some(true));
        } else {
            panic!("not an element");
        }
    }
}

impl<'a> Default for Dom<'a> {
    fn default() -> Self {
        let mut dom = Dom {
            arena: Arena::new(),
            errors: Vec::new(),
            quirks_mode: QuirksMode::NoQuirks,
            document: unsafe { mem::uninitialized() },
        };
        dom.document = dom.create_tree_node(Node::Document);
        dom
    }
}

impl<'a> ParseResult for Dom<'a> {
    type Sink = Dom<'a>;
    fn get_result(sink: Dom<'a>) -> Dom<'a> { sink }
}
