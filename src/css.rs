use std::ops::Deref;

use cssparser;
use ego_tree::NodeRef;
use selectors;
use selectors::parser::AttrSelector;
use string_cache::{QualName, Atom, Namespace};

use super::*;

impl Html {
    /// You know.
    pub fn css(&self, s: &str) -> Option<NodeRef<HtmlNode>> {
        let mut parser = cssparser::Parser::new(s);
        let ctx = selectors::parser::ParserContext::new();
        let sels = selectors::parser::parse_selector_list(&ctx, &mut parser).unwrap();
        self.tree.nodes()
            .filter(|node| node.value().is_element())
            .find(|node| selectors::matching::matches(&sels, &Handle(*node), None))
    }
}

pub struct Handle<'a>(NodeRef<'a, HtmlNode>);

impl<'a> Deref for Handle<'a> {
    type Target = NodeRef<'a, HtmlNode>;

    fn deref(&self) -> &NodeRef<'a, HtmlNode> {
        &self.0
    }
}

impl<'a> selectors::Element for Handle<'a> {
    fn parent_element(&self) -> Option<Self> {
        self.parent().and_then(|parent| {
            if parent.value().is_element() {
                Some(Handle(parent))
            } else {
                None
            }
        })
    }

    fn first_child_element(&self) -> Option<Self> {
        self.children()
            .find(|child| child.value().is_element())
            .map(Handle)
    }

    fn last_child_element(&self) -> Option<Self> {
        self.children()
            .rev()
            .find(|child| child.value().is_element())
            .map(Handle)
    }

    fn prev_sibling_element(&self) -> Option<Self> {
        self.prev_siblings()
            .find(|child| child.value().is_element())
            .map(Handle)
    }

    fn next_sibling_element(&self) -> Option<Self> {
        self.next_siblings()
            .find(|child| child.value().is_element())
            .map(Handle)
    }

    fn is_html_element_in_html_document(&self) -> bool {
        // FIXME: ???
        true
    }

    fn get_local_name(&self) -> &Atom {
        &self.value().as_element().unwrap().name.local
    }

    fn get_namespace(&self) -> &Namespace {
        &self.value().as_element().unwrap().name.ns
    }

    fn get_active_state(&self) -> bool {
        false
    }

    fn get_focus_state(&self) -> bool {
        false
    }

    fn get_hover_state(&self) -> bool {
        false
    }

    fn get_enabled_state(&self) -> bool {
        // FIXME: do we need to check attrs?
        false
    }

    fn get_disabled_state(&self) -> bool {
        // FIXME: do we need to check attrs?
        false
    }

    fn get_checked_state(&self) -> bool {
        // FIXME: do we need to check attrs?
        false
    }

    fn get_intermediate_state(&self) -> bool {
        // FIXME: what even is this?
        false
    }

    fn get_id(&self) -> Option<Atom> {
        self.value()
            .as_element()
            .unwrap()
            .attrs
            .get(&qualname!("", "id"))
            .map(|tendril| &tendril[..])
            .map(Into::into)
    }

    fn has_class(&self, name: &Atom) -> bool {
        // FIXME: ouch.
        self.value()
            .as_element()
            .unwrap()
            .attrs
            .get(&qualname!("", "class"))
            .into_iter()
            .map(Deref::deref)
            .flat_map(str::split_whitespace)
            .any(|class| class == name.deref())
    }

    fn match_attr<F>(&self, attr: &AttrSelector, test: F) -> bool where F: Fn(&str) -> bool {
        // FIXME: handle AttrSelector properly.
        self.value()
            .as_element()
            .unwrap()
            .attrs
            .get(&QualName::new(ns!(), attr.lower_name.clone()))
            .map(Deref::deref)
            .map(test)
            .unwrap_or(false)
    }

    fn is_empty(&self) -> bool {
        !self.children()
            .any(|child| child.value().is_element() || child.value().is_text())
    }

    fn is_root(&self) -> bool {
        // FIXME: fragments.
        self.parent()
            .map(|parent| parent.value().is_document())
            .unwrap_or(false)
    }

    fn is_link(&self) -> bool {
        unimplemented!()
    }

    fn each_class<F>(&self, mut callback: F) where F: FnMut(&Atom) {
        // FIXME: lord.
        let classes = self.value()
            .as_element()
            .unwrap()
            .attrs
            .get(&qualname!("", "class"))
            .into_iter()
            .map(Deref::deref)
            .flat_map(str::split_whitespace);
        for class in classes {
            callback(&Atom::from(class));
        }
    }
}
