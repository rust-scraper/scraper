use std::io::{Error, Write};

use ego_tree::iter::Edge;
use html5ever::serialize::{Serializable, Serializer, TraversalScope};

use {ElementRef, Node};

impl<'a> Serializable for ElementRef<'a> {
    fn serialize<'w, W: Write>(
        &self,
        serializer: &mut Serializer<'w, W>,
        traversal_scope: TraversalScope,
    ) -> Result<(), Error> {
        for edge in self.traverse() {
            match edge {
                Edge::Open(node) => {
                    if node == **self && traversal_scope == TraversalScope::ChildrenOnly {
                        continue;
                    }

                    match *node.value() {
                        Node::Doctype(ref doctype) => {
                            try!(serializer.write_doctype(doctype.name()));
                        },
                        Node::Comment(ref comment) => {
                            try!(serializer.write_comment(comment));
                        },
                        Node::Text(ref text) => {
                            try!(serializer.write_text(text));
                        },
                        Node::Element(ref elem) => {
                            let attrs = elem.attrs.iter().map(|(k, v)| (k, &v[..]));
                            try!(serializer.start_elem(elem.name.clone(), attrs));
                        },
                        _ => (),
                    }
                },

                Edge::Close(node) => {
                    if node == **self && traversal_scope == TraversalScope::ChildrenOnly {
                        continue;
                    }

                    if let Some(elem) = node.value().as_element() {
                        try!(serializer.end_elem(elem.name.clone()));
                    }
                },
            }
        }

        Ok(())
    }
}
