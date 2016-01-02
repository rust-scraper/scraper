//! DOM implementation.

use std::borrow::Cow;
use std::fmt;
use std::ops::Deref;

use html5ever::Attribute;
use html5ever::tree_builder::{TreeSink, QuirksMode, NodeOrText};
use string_cache::QualName;
use tendril::StrTendril;
use typed_arena::Arena;

/// Arena-allocated DOM.
pub struct Dom<'a> {
    arena: Arena<TreeNode<'a>>,

    /// Parse errors.
    pub errors: Vec<Cow<'static, str>>,

    /// The document root node.
    pub document: &'a TreeNode<'a>,
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

/// A reference to a `TreeNode`.
#[derive(Debug, Clone)]
pub struct Handle<'a>(&'a TreeNode<'a>);
impl<'a> Deref for Handle<'a> {
    type Target = TreeNode<'a>;
    fn deref(&self) -> &TreeNode<'a> { self.0 }
}

impl<'a> fmt::Debug for Dom<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.debug_struct("Dom")
            .field("document", &self.document)
            .finish()
    }
}

#[allow(unused_variables)]
impl<'a> TreeSink for Dom<'a> {
    type Handle = Handle<'a>;

    fn parse_error(&mut self, msg: Cow<'static, str>) {
        self.errors.push(msg);
    }

    fn get_document(&mut self) -> Handle<'a> {
        Handle(self.document)
    }

    fn get_template_contents(&self, target: Self::Handle) -> Self::Handle {
        unimplemented!()
    }

    #[allow(trivial_casts)]
    fn same_node(&self, x: Handle<'a>, y: Handle<'a>) -> bool {
        x.0 as *const _ == y.0 as *const _
    }

    fn elem_name(&self, target: Self::Handle) -> QualName {
        unimplemented!()
    }

    fn set_quirks_mode(&mut self, mode: QuirksMode) {
        unimplemented!();
    }

    fn create_element(&mut self, name: QualName, attrs: Vec<Attribute>) -> Self::Handle {
        unimplemented!()
    }

    fn create_comment(&mut self, text: StrTendril) -> Self::Handle {
        unimplemented!()
    }

    fn append(&mut self, parent: Self::Handle, child: NodeOrText<Self::Handle>) {
        unimplemented!();
    }

    fn append_before_sibling(&mut self, sibling: Self::Handle, new_node: NodeOrText<Self::Handle>) -> Result<(), NodeOrText<Self::Handle>> {
        unimplemented!()
    }

    fn append_doctype_to_document(&mut self, name: StrTendril, public_id: StrTendril, system_id: StrTendril) {
        unimplemented!();
    }

    fn add_attrs_if_missing(&mut self, target: Self::Handle, attrs: Vec<Attribute>) {
        unimplemented!();
    }

    fn remove_from_parent(&mut self, target: Self::Handle) {
        unimplemented!();
    }

    fn reparent_children(&mut self, node: Self::Handle, new_parent: Self::Handle) {
        unimplemented!();
    }

    fn mark_script_already_started(&mut self, node: Self::Handle) {
        unimplemented!();
    }
}
