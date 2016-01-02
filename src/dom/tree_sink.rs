//! `TreeSink` implementation.

use super::*;

use std::borrow::Cow;
use std::cell::RefCell;
use std::ops::Deref;

use html5ever::Attribute;
use html5ever::driver::ParseResult;
use html5ever::tree_builder::{TreeSink, QuirksMode, NodeOrText};
use string_cache::QualName;
use tendril::StrTendril;

/// A reference to a `TreeNode`.
#[derive(Debug, Clone)]
pub struct Handle<'a>(&'a TreeNode<'a>);

impl<'a> Deref for Handle<'a> {
    type Target = TreeNode<'a>;
    fn deref(&self) -> &TreeNode<'a> { self.0 }
}

fn append_child<'a>(parent: &'a TreeNode<'a>, child: &'a TreeNode<'a>) {
    if let Some((first, last)) = parent.children.get() {
        last.next_sibling.set(Some(child));
        child.prev_sibling.set(Some(last));
        parent.children.set(Some((first, child)));
    } else {
        parent.children.set(Some((child, child)));
    }
    child.parent.set(Some(parent));
}

fn insert_before<'a>(node: &'a TreeNode<'a>, new_node: &'a TreeNode<'a>) {
    let parent = node.parent().unwrap();

    if let Some(prev) = node.prev_sibling() {
        new_node.prev_sibling.set(Some(prev));
        new_node.next_sibling.set(Some(node));
        prev.next_sibling.set(Some(new_node));
        node.prev_sibling.set(Some(new_node));
    } else {
        new_node.next_sibling.set(Some(node));
        node.prev_sibling.set(Some(new_node));
        let last = parent.children.get().unwrap().1;
        parent.children.set(Some((new_node, last)));
    }

    new_node.parent.set(Some(parent));
}

impl<'a> TreeSink for Dom<'a> {
    type Handle = Handle<'a>;

    // Signal a parse error.
    fn parse_error(&mut self, msg: Cow<'static, str>) {
        self.errors.push(msg);
    }

    // Get a handle to the Document node.
    fn get_document(&mut self) -> Handle<'a> {
        Handle(self.document)
    }

    // Get a handle to a template's template contents.
    //
    // The tree builder promises this will never be called with something else than a template
    // element.
    fn get_template_contents(&self, target: Handle<'a>) -> Handle<'a> {
        let Handle(node) = target;
        if let Node::Element(ref element) = node.node {
            Handle(element.template_contents.unwrap())
        } else {
            panic!("not an element")
        }
    }

    // Do two handles refer to the same node?
    fn same_node(&self, x: Handle<'a>, y: Handle<'a>) -> bool {
        TreeNode::same(x.0, y.0)
    }

    // What is the name of this element?
    //
    // Should never be called on a non-element node; feel free to panic!.
    fn elem_name(&self, target: Handle<'a>) -> QualName {
        let Handle(node) = target;
        if let Node::Element(ref element) = node.node {
            element.name.clone()
        } else {
            panic!("not an element")
        }
    }

    // Set the document's quirks mode.
    fn set_quirks_mode(&mut self, mode: QuirksMode) {
        self.quirks_mode = mode;
    }

    // Create an element.
    //
    // When creating a template element (name == qualname!(html, "template")), an associated
    // document fragment called the "template contents" should also be created. Later calls to
    // self.get_template_contents() with that given element return it.
    fn create_element(&mut self, name: QualName, attrs: Vec<Attribute>) -> Handle<'a> {
        let attrs = attrs.into_iter()
            .map(|a| (a.name, a.value))
            .collect();

        if name == qualname!(html, "template") {
            let contents = self.new_tree_node(Node::Document);
            let element = Element {
                name: name,
                attrs: RefCell::new(attrs),
                template_contents: Some(contents),
            };
            Handle(self.new_tree_node(Node::Element(element)))
        } else {
            let element = Element {
                name: name,
                attrs: RefCell::new(attrs),
                template_contents: None,
            };
            Handle(self.new_tree_node(Node::Element(element)))
        }
    }

    // Create a comment node.
    fn create_comment(&mut self, text: StrTendril) -> Handle<'a> {
        Handle(self.new_tree_node(Node::Comment(text)))
    }

    // Append a node as the last child of the given node. If this would produce adjacent sibling
    // text nodes, it should concatenate the text instead.
    //
    // The child node will not already have a parent.
    fn append(&mut self, parent: Handle<'a>, child: NodeOrText<Handle<'a>>) {
        let Handle(parent) = parent;
        match child {
            NodeOrText::AppendNode(Handle(node)) => {
                append_child(parent, node);
            },

            NodeOrText::AppendText(text) => {
                let last_child = parent.last_child();
                if let Some(&TreeNode { node: Node::Text(ref tendril), .. }) = last_child {
                    tendril.borrow_mut().push_tendril(&text);
                } else {
                    let node = Node::Text(RefCell::new(text));
                    append_child(parent, self.new_tree_node(node));
                }
            },
        }
    }

    // Append a node as the sibling immediately before the given node. If that node has no parent,
    // do nothing and return Err(new_node).
    //
    // The tree builder promises that sibling is not a text node. However its old previous sibling,
    // which would become the new node's previous sibling, could be a text node. If the new node is
    // also a text node, the two should be merged, as in the behavior of append.
    //
    // NB: new_node may have an old parent, from which it should be removed.
    fn append_before_sibling(
        &mut self,
        sibling: Handle<'a>,
        new_node: NodeOrText<Handle<'a>>
    ) -> Result<(), NodeOrText<Handle<'a>>> {
        let Handle(sibling) = sibling;
        if sibling.parent().is_none() {
            return Err(new_node)
        }

        match new_node {
            NodeOrText::AppendNode(Handle(node)) => {
                self.remove_from_parent(Handle(node));
                insert_before(sibling, node);
            },

            NodeOrText::AppendText(text) => {
                let prev = sibling.prev_sibling();
                if let Some(&TreeNode { node: Node::Text(ref tendril), .. }) = prev {
                    tendril.borrow_mut().push_tendril(&text);
                } else {
                    let node = Node::Text(RefCell::new(text));
                    insert_before(sibling, self.new_tree_node(node));
                }
            },
        }

        Ok(())
    }

    // Append a DOCTYPE element to the Document node.
    fn append_doctype_to_document(
        &mut self,
        name: StrTendril,
        public_id: StrTendril,
        system_id: StrTendril
    ) {
        let doctype = Doctype {
            name: name,
            public_id: public_id,
            system_id: system_id,
        };
        append_child(self.document, self.new_tree_node(Node::Doctype(doctype)));
    }

    // Add each attribute to the given element, if no attribute with that name already exists. The
    // tree builder promises this will never be called with something else than an element.
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

    // Detach the given node from its parent.
    fn remove_from_parent(&mut self, target: Handle<'a>) {
        let Handle(node) = target;
        let parent = match node.parent() {
            Some(p) => p,
            None => return,
        };
        let prev_sibling = node.prev_sibling();
        let next_sibling = node.next_sibling();

        // Update sibling refs.
        if let Some(prev) = prev_sibling {
            prev.next_sibling.set(node.next_sibling());
        }
        if let Some(next) = next_sibling {
            next.prev_sibling.set(node.prev_sibling());
        }

        // Update parent refs.
        if prev_sibling.is_none() && next_sibling.is_none() {
            parent.children.set(None);
        } else {
            let mut parent_children = parent.children.get().unwrap();
            if TreeNode::same(parent_children.0, node) {
                parent_children.0 = next_sibling.unwrap();
            }
            if TreeNode::same(parent_children.1, node) {
                parent_children.1 = prev_sibling.unwrap();
            }
            parent.children.set(Some(parent_children));
        }

        // Orphan node.
        node.parent.set(None);
        node.next_sibling.set(None);
        node.prev_sibling.set(None);
    }

    // Remove all the children from node and append them to new_parent.
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
            child = node.next_sibling();
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

    // Mark a HTML <script> element as "already started".
    fn mark_script_already_started(&mut self, _node: Handle<'a>) {
        // Unnecessary.
    }
}

impl<'a> ParseResult for Dom<'a> {
    type Sink = Dom<'a>;
    fn get_result(sink: Dom<'a>) -> Dom<'a> { sink }
}
