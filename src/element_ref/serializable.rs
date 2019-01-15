use std::io::Error;

use ego_tree::iter::Edge;
use html5ever::serialize::{Serialize, Serializer, TraversalScope};

use crate::{ElementRef, Node};


impl<'a> Serialize for ElementRef<'a> {
    fn serialize<S: Serializer>(
        &self,
        serializer: &mut S,
        traversal_scope: TraversalScope,
    ) -> Result<(), Error> {
        for edge in self.traverse() {
            match edge {
                Edge::Open(node) => {
                    if node == **self && traversal_scope == TraversalScope::ChildrenOnly(None) {
                        continue;
                    }

                    match *node.value() {
                        Node::Doctype(ref doctype) => {
                            r#try!(serializer.write_doctype(doctype.name()));
                        },
                        Node::Comment(ref comment) => {
                            r#try!(serializer.write_comment(comment));
                        },
                        Node::Text(ref text) => {
                            r#try!(serializer.write_text(text));
                        },
                        Node::Element(ref elem) => {
                            let attrs = elem.attrs.iter().map(|(k, v)| (k, &v[..]));
                            r#try!(serializer.start_elem(elem.name.clone(), attrs));
                        },
                        _ => (),
                    }
                },

                Edge::Close(node) => {
                    if node == **self && traversal_scope == TraversalScope::ChildrenOnly(None) {
                        continue;
                    }

                    if let Some(elem) = node.value().as_element() {
                        r#try!(serializer.end_elem(elem.name.clone()));
                    }
                },
            }
        }

        Ok(())
    }
}
