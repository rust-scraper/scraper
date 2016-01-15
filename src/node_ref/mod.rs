//! Node references.

use std::ops::Deref;

use ego_tree;

use node::Node;

/// Wrapper around a reference to an HTML node.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NodeRef<'a>(pub ego_tree::NodeRef<'a, Node>);

impl<'a> Deref for NodeRef<'a> {
    type Target = ego_tree::NodeRef<'a, Node>;

    fn deref(&self) -> &ego_tree::NodeRef<'a, Node> {
        &self.0
    }
}

mod element;
