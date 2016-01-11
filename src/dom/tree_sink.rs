//! `TreeSink` implementation.

use super::*;

use std::borrow::Cow;

use ego_tree::NodeId;
use html5ever::Attribute;
use html5ever::tree_builder::{TreeSink, QuirksMode, NodeOrText};
use string_cache::QualName;
use tendril::StrTendril;

impl TreeSink for Dom {
    type Handle = NodeId<Node>;

    // Signal a parse error.
    fn parse_error(&mut self, msg: Cow<'static, str>) {
        self.errors.push(msg);
    }

    // Get a handle to the Document node.
    fn get_document(&mut self) -> NodeId<Node> {
        self.tree.root().id()
    }

    // Get a handle to a template's template contents.
    //
    // The tree builder promises this will never be called with something else than a template
    // element.
    fn get_template_contents(&self, target: NodeId<Node>) -> NodeId<Node> {
        let node = self.tree.get(target);
        if let &Node::Element(ref element) = node.value() {
            element.template_contents_id.unwrap()
        } else {
            panic!("not an element")
        }
    }

    // Do two handles refer to the same node?
    fn same_node(&self, x: NodeId<Node>, y: NodeId<Node>) -> bool {
        x == y
    }

    // What is the name of this element?
    //
    // Should never be called on a non-element node; feel free to panic!.
    fn elem_name(&self, target: NodeId<Node>) -> QualName {
        let node = self.tree.get(target);
        if let &Node::Element(ref element) = node.value() {
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
    fn create_element(&mut self, name: QualName, attrs: Vec<Attribute>) -> NodeId<Node> {
        let attrs = attrs.into_iter()
            .map(|a| (a.name, a.value))
            .collect();

        let mut element = Element {
            name: name,
            attrs: attrs,
            template_contents_id: None,
        };

        if element.name == qualname!(html, "template") {
            element.template_contents_id = Some(self.tree.orphan(Node::Document).id());
        }

        self.tree.orphan(Node::Element(element)).id()
    }

    // Create a comment node.
    fn create_comment(&mut self, text: StrTendril) -> NodeId<Node> {
        self.tree.orphan(Node::Comment(text)).id()
    }

    // Append a node as the last child of the given node. If this would produce adjacent sibling
    // text nodes, it should concatenate the text instead.
    //
    // The child node will not already have a parent.
    fn append(&mut self, parent: NodeId<Node>, child: NodeOrText<NodeId<Node>>) {
        let mut parent = self.tree.get_mut(parent);

        match child {
            NodeOrText::AppendNode(id) => {
                unsafe { parent.append_id(id); }
            },

            NodeOrText::AppendText(text) => {
                let can_append = {
                    if let Some(mut last_child) = parent.last_child() {
                        match *last_child.value() {
                            Node::Text(_) => true,
                            _ => false,
                        }
                    } else {
                        false
                    }
                };

                if can_append {
                    let mut last_child = parent.last_child().unwrap();
                    if let &mut Node::Text(ref mut tendril) = last_child.value() {
                        tendril.push_tendril(&text);
                    } else {
                        unreachable!();
                    }
                } else {
                    parent.append(Node::Text(text));
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
        sibling: NodeId<Node>,
        new_node: NodeOrText<NodeId<Node>>
    ) -> Result<(), NodeOrText<NodeId<Node>>> {
        if let NodeOrText::AppendNode(node_id) = new_node {
            let mut new_node = self.tree.get_mut(node_id);
            new_node.detach();
        }

        let mut sibling = self.tree.get_mut(sibling);
        if sibling.parent().is_none() {
            return Err(new_node);
        }

        match new_node {
            NodeOrText::AppendNode(id) => {
                unsafe { sibling.insert_id_before(id); }
            },

            NodeOrText::AppendText(text) => {
                let can_append = {
                    if let Some(mut prev_sibling) = sibling.prev_sibling() {
                        match *prev_sibling.value() {
                            Node::Text(_) => true,
                            _ => false,
                        }
                    } else {
                        false
                    }
                };

                if can_append {
                    let mut prev_sibling = sibling.prev_sibling().unwrap();
                    if let &mut Node::Text(ref mut tendril) = prev_sibling.value() {
                        tendril.push_tendril(&text);
                    } else {
                        unreachable!();
                    }
                } else {
                    sibling.insert_before(Node::Text(text));
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
        self.tree.root_mut().append(Node::Doctype(doctype));
    }

    // Add each attribute to the given element, if no attribute with that name already exists. The
    // tree builder promises this will never be called with something else than an element.
    fn add_attrs_if_missing(&mut self, target: NodeId<Node>, attrs: Vec<Attribute>) {
        let mut node = self.tree.get_mut(target);
        if let &mut Node::Element(ref mut element) = node.value() {
            for attr in attrs {
                if !element.attrs.contains_key(&attr.name) {
                    element.attrs.insert(attr.name, attr.value);
                }
            }
        } else {
            panic!("not an element");
        }
    }

    // Detach the given node from its parent.
    fn remove_from_parent(&mut self, target: NodeId<Node>) {
        self.tree.get_mut(target).detach();
    }

    // Remove all the children from node and append them to new_parent.
    fn reparent_children(&mut self, node: NodeId<Node>, new_parent: NodeId<Node>) {
        unsafe { self.tree.get_mut(new_parent).reparent_from_id_append(node); }
    }

    // Mark a HTML <script> element as "already started".
    fn mark_script_already_started(&mut self, _node: NodeId<Node>) {
        // Unnecessary.
    }
}
