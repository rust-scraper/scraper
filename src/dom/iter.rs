//! `TreeNode` iterators.

use super::TreeNode;

/// Iterator over `TreeNode` siblings.
#[derive(Debug)]
pub struct Siblings<'a> {
    node: Option<&'a TreeNode<'a>>,
}

impl<'a> Iterator for Siblings<'a> {
    type Item = &'a TreeNode<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.node;
        self.node = node.and_then(TreeNode::next_sibling);
        node
    }
}

/// Iterator over `TreeNode` children.
#[derive(Debug)]
pub struct Children<'a> {
    front: Option<&'a TreeNode<'a>>,
    back: Option<&'a TreeNode<'a>>,
}

impl<'a> Children<'a> {
    fn end_if_same(&mut self) {
        if let Some(front) = self.front {
            if let Some(back) = self.back {
                if TreeNode::same(front, back) {
                    self.front = None;
                    self.back = None;
                }
            }
        }
    }
}

impl<'a> Iterator for Children<'a> {
    type Item = &'a TreeNode<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.front;
        self.end_if_same();
        self.front = self.front.and_then(TreeNode::next_sibling);
        node
    }
}

impl<'a> DoubleEndedIterator for Children<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let node = self.back;
        self.end_if_same();
        self.back = self.back.and_then(TreeNode::prev_sibling);
        node
    }
}

/// Iterator over `TreeNode` ancestors.
#[derive(Debug)]
pub struct Ancestors<'a> {
    node: Option<&'a TreeNode<'a>>,
}

impl<'a> Iterator for Ancestors<'a> {
    type Item = &'a TreeNode<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.node;
        self.node = node.and_then(TreeNode::parent);
        node
    }
}

impl<'a> TreeNode<'a> {
    /// Returns an iterator over this node's siblings, starting with this node.
    pub fn iter(&'a self) -> Siblings {
        Siblings { node: Some(self) }
    }

    /// Returns an iterator over this node's children.
    pub fn children(&self) -> Children<'a> {
        if let Some((front, back)) = self.children.get() {
            Children {
                front: Some(front),
                back: Some(back),
            }
        } else {
            Children {
                front: None,
                back: None,
            }
        }
    }

    /// Returns an iterator over this node's ancestors, starting with this node.
    pub fn ancestors(&'a self) -> Ancestors {
        Ancestors { node: Some(self) }
    }
}
