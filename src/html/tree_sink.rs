use std::borrow::Cow;

use ego_tree::NodeId;
use html5ever::tree_builder::{TreeSink, QuirksMode, NodeOrText};
use html5ever::Attribute;
use string_cache::QualName;
use tendril::StrTendril;

use super::Html;
use node::{Node, Doctype, Comment, Text, Element};

type Handle = NodeId<Node>;

impl TreeSink for Html {
    type Output = Self;
    type Handle = NodeId<Node>;

    fn finish(self) -> Self { self }

    // Signal a parse error.
    fn parse_error(&mut self, msg: Cow<'static, str>) {
        self.errors.push(msg);
    }

    // Set the document's quirks mode.
    fn set_quirks_mode(&mut self, mode: QuirksMode) {
        self.quirks_mode = mode;
    }

    // Get a handle to the Document node.
    fn get_document(&mut self) -> Handle {
        self.tree.root().id()
    }

    // Do two handles refer to the same node?
    fn same_node(&self, x: Handle, y: Handle) -> bool {
        x == y
    }

    // What is the name of this element?
    //
    // Should never be called on a non-element node; feel free to panic!.
    fn elem_name(&self, target: Handle) -> QualName {
        self.tree.get(target)
            .value()
            .as_element()
            .unwrap()
            .name
            .clone()
    }

    // Create an element.
    //
    // When creating a template element (name == qualname!(html, "template")), an associated
    // document fragment called the "template contents" should also be created. Later calls to
    // self.get_template_contents() with that given element return it.
    fn create_element(&mut self, name: QualName, attrs: Vec<Attribute>) -> Handle {
        let attrs = attrs.into_iter()
            .map(|a| (a.name, a.value))
            .collect();
        self.tree.orphan(Node::Element(Element::new(name, attrs))).id()
    }

    // Create a comment node.
    fn create_comment(&mut self, text: StrTendril) -> Handle {
        self.tree.orphan(Node::Comment(Comment { comment: text })).id()
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

    // Append a node as the last child of the given node. If this would produce adjacent sibling
    // text nodes, it should concatenate the text instead.
    //
    // The child node will not already have a parent.
    fn append(&mut self, parent: Handle, child: NodeOrText<Handle>) {
        let mut parent = self.tree.get_mut(parent);

        match child {
            NodeOrText::AppendNode(id) => {
                unsafe { parent.append_id(id); }
            },

            NodeOrText::AppendText(text) => {
                let can_concat = parent.last_child()
                    .map(|mut n| n.value().is_text())
                    .unwrap_or(false);

                if can_concat {
                    let mut last_child = parent.last_child().unwrap();
                    match *last_child.value() {
                        Node::Text(ref mut t) => t.text.push_tendril(&text),
                        _ => unreachable!(),
                    }
                } else {
                    parent.append(Node::Text(Text { text: text }));
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
        sibling: Handle,
        new_node: NodeOrText<Handle>
    ) -> Result<(), NodeOrText<Handle>> {
        if let NodeOrText::AppendNode(id) = new_node {
            self.tree.get_mut(id).detach();
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
                let can_concat = sibling.prev_sibling()
                    .map(|mut n| n.value().is_text())
                    .unwrap_or(false);

                if can_concat {
                    let mut prev_sibling = sibling.prev_sibling().unwrap();
                    match *prev_sibling.value() {
                        Node::Text(ref mut t) => t.text.push_tendril(&text),
                        _ => unreachable!(),
                    }
                } else {
                    sibling.insert_before(Node::Text(Text { text: text }));
                }
            },
        }

        Ok(())
    }

    // Detach the given node from its parent.
    fn remove_from_parent(&mut self, target: Handle) {
        self.tree.get_mut(target).detach();
    }

    // Remove all the children from node and append them to new_parent.
    fn reparent_children(&mut self, node: Handle, new_parent: Handle) {
        unsafe { self.tree.get_mut(new_parent).reparent_from_id_append(node); }
    }

    // Add each attribute to the given element, if no attribute with that name already exists. The
    // tree builder promises this will never be called with something else than an element.
    fn add_attrs_if_missing(&mut self, target: Handle, attrs: Vec<Attribute>) {
        let mut node = self.tree.get_mut(target);
        let mut element = match *node.value() {
            Node::Element(ref mut e) => e,
            _ => unreachable!(),
        };

        for attr in attrs {
            if !element.attrs.contains_key(&attr.name) {
                element.attrs.insert(attr.name, attr.value);
            }
        }
    }

    // Get a handle to a template's template contents.
    //
    // The tree builder promises this will never be called with something else than a template
    // element.
    fn get_template_contents(&self, _target: Handle) -> Handle {
        unimplemented!()
    }

    // Mark a HTML <script> element as "already started".
    fn mark_script_already_started(&mut self, _node: Handle) {
    }
}
