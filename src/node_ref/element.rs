use std::ops::Deref;

use selectors::Element;
use selectors::parser::AttrSelector;
use string_cache::{QualName, Atom, Namespace};

use super::NodeRef;

impl<'a> Element for NodeRef<'a> {
    fn parent_element(&self) -> Option<Self> {
        self.parent().and_then(|parent| {
            if parent.value().is_element() {
                Some(NodeRef(parent))
            } else {
                None
            }
        })
    }

    fn first_child_element(&self) -> Option<Self> {
        self.children()
            .find(|child| child.value().is_element())
            .map(NodeRef)
    }

    fn last_child_element(&self) -> Option<Self> {
        self.children()
            .rev()
            .find(|child| child.value().is_element())
            .map(NodeRef)
    }

    fn prev_sibling_element(&self) -> Option<Self> {
        self.prev_siblings()
            .find(|child| child.value().is_element())
            .map(NodeRef)
    }

    fn next_sibling_element(&self) -> Option<Self> {
        self.next_siblings()
            .find(|child| child.value().is_element())
            .map(NodeRef)
    }

    fn is_html_element_in_html_document(&self) -> bool {
        // FIXME: Is there more to this?
        self.value()
            .as_element()
            .map(|element| element.name.ns == ns!(html))
            .unwrap_or(false)
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
        false
    }

    fn get_disabled_state(&self) -> bool {
        false
    }

    fn get_checked_state(&self) -> bool {
        false
    }

    fn get_intermediate_state(&self) -> bool {
        false
    }

    fn get_id(&self) -> Option<Atom> {
        self.value()
            .as_element()
            .unwrap()
            .id
            .clone()
    }

    fn has_class(&self, name: &Atom) -> bool {
        self.value()
            .as_element()
            .unwrap()
            .classes
            .contains(name)
    }

    fn match_attr<F>(&self, attr: &AttrSelector, test: F) -> bool where F: Fn(&str) -> bool {
        self.value()
            .as_element()
            .unwrap()
            .attrs
            .get(&QualName::new(ns!(), attr.name.clone()))
            .map(Deref::deref)
            .map(test)
            .unwrap_or(false)
    }

    fn is_empty(&self) -> bool {
        !self.children()
            .any(|child| child.value().is_element() || child.value().is_text())
    }

    fn is_root(&self) -> bool {
        self.parent()
            .map(|parent| parent.value().is_document())
            .unwrap_or(false)
    }

    fn is_link(&self) -> bool {
        false
    }

    fn each_class<F>(&self, mut callback: F) where F: FnMut(&Atom) {
        for class in &self.value().as_element().unwrap().classes {
            callback(class);
        }
    }
}
