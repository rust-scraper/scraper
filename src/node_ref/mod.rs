//! Node references.

use std::ops::Deref;

use ego_tree;

use Node;

/// Wrapper around a reference to an HTML node.
///
/// This wrapper implements the `Element` trait from the `selectors` crate, which allows it to be
/// matched against CSS selectors.
///
/// Note that this implementation will never match against these pseudo-classes:
///
/// - `:active`
/// - `:focus`
/// - `:hover`
/// - `:enabled`
/// - `:disabled`
/// - `:checked`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NodeRef<'a>(pub ego_tree::NodeRef<'a, Node>);

impl<'a> Deref for NodeRef<'a> {
    type Target = ego_tree::NodeRef<'a, Node>;

    fn deref(&self) -> &ego_tree::NodeRef<'a, Node> {
        &self.0
    }
}

mod element;
