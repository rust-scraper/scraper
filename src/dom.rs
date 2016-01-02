//! DOM implementation.

use std::fmt;

use typed_arena::Arena;

/// Arena-allocated DOM.
pub struct Dom<'a> {
    arena: Arena<TreeNode<'a>>,

    /// The root node.
    pub root: &'a TreeNode<'a>,
}

/// A node in the DOM tree.
#[derive(Debug)]
pub struct TreeNode<'a> {
    /// The DOM node.
    pub node: (),

    /// The parent node.
    pub parent: Option<&'a TreeNode<'a>>,

    /// The first and last children.
    pub children: Option<(&'a TreeNode<'a>, &'a TreeNode<'a>)>,

    /// The next sibling.
    pub next_sibling: Option<&'a TreeNode<'a>>,

    /// The previous sibling.
    pub prev_sibling: Option<&'a TreeNode<'a>>,
}

impl<'a> fmt::Debug for Dom<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.debug_struct("Dom")
            .field("root", &self.root)
            .finish()
    }
}
