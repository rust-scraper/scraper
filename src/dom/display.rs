use super::{TreeNode, Node};

use std::fmt::{Display, Formatter, Error};
use std::iter;

// FIXME: Copy of str::escape_default from std, which is currently unstable.
fn escape_default(s: &str) -> String {
    s.chars().flat_map(|c| c.escape_default()).collect()
}

impl<'a> Display for Node<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            Node::Document => write!(f, "Document"),

            Node::Doctype(ref doctype) => {
                write!(
                    f,
                    "<!DOCTYPE {} \"{}\" \"{}\">",
                    doctype.name,
                    escape_default(&doctype.public_id),
                    escape_default(&doctype.system_id)
                )
            },

            Node::Comment(ref text) => write!(f, "<!-- \"{}\" -->", escape_default(text)),

            Node::Text(ref text) => write!(f, "\"{}\"", escape_default(&text.borrow())),

            Node::Element(ref element) => {
                try!(write!(f, "<{}", element.name.local));

                for (name, value) in element.attrs.borrow().iter() {
                    try!(write!(f, " {}=\"{}\"", name.local, escape_default(value)));
                }

                write!(f, ">")
            },
        }
    }
}

impl<'a> TreeNode<'a> {
    fn fmt_tree(&self, f: &mut Formatter, level: usize) -> Result<(), Error> {
        let indent: String = iter::repeat(" ").take(level).collect();
        try!(write!(f, "{}{}\n", indent, self.node));

        for child in self.children() {
            try!(child.fmt_tree(f, level + 2));
        }

        Ok(())
    }
}

impl<'a> Display for TreeNode<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        self.fmt_tree(f, 0)
    }
}
