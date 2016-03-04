use std::ops::Deref;

use selectors::Element;
use selectors::parser::AttrSelector;
use string_cache::{QualName, Atom, Namespace};

use super::ElementRef;
use selector::Simple;

/// Note: will never match against non-tree-structure pseudo-classes.
impl<'a> Element for ElementRef<'a> {
    type Impl = Simple;

    fn parent_element(&self) -> Option<Self> {
        self.parent().and_then(ElementRef::wrap)
    }

    fn first_child_element(&self) -> Option<Self> {
        self.children()
            .find(|child| child.value().is_element())
            .map(ElementRef::new)
    }

    fn last_child_element(&self) -> Option<Self> {
        self.children()
            .rev()
            .find(|child| child.value().is_element())
            .map(ElementRef::new)
    }

    fn prev_sibling_element(&self) -> Option<Self> {
        self.prev_siblings()
            .find(|sibling| sibling.value().is_element())
            .map(ElementRef::new)
    }

    fn next_sibling_element(&self) -> Option<Self> {
        self.next_siblings()
            .find(|sibling| sibling.value().is_element())
            .map(ElementRef::new)
    }

    fn is_html_element_in_html_document(&self) -> bool {
        // FIXME: Is there more to this?
        self.value().name.ns == ns!(html)
    }

    fn get_local_name(&self) -> &Atom {
        &self.value().name.local
    }

    fn get_namespace(&self) -> &Namespace {
        &self.value().name.ns
    }

    fn match_non_ts_pseudo_class(&self, _pc: ()) -> bool {
        false
    }

    fn get_id(&self) -> Option<Atom> {
        self.value().id.clone()
    }

    fn has_class(&self, name: &Atom) -> bool {
        self.value().classes.contains(name)
    }

    fn match_attr<F>(&self, attr: &AttrSelector, test: F) -> bool where F: Fn(&str) -> bool {
        self.value()
            .attrs
            .get(&QualName::new(ns!(), attr.name.clone()))
            .map(Deref::deref)
            .map_or(false, test)
    }

    fn is_empty(&self) -> bool {
        !self.children()
            .any(|child| child.value().is_element() || child.value().is_text())
    }

    fn is_root(&self) -> bool {
        self.parent()
            .map_or(false, |parent| parent.value().is_document())
    }

    fn each_class<F>(&self, mut callback: F) where F: FnMut(&Atom) {
        for class in &self.value().classes {
            callback(class);
        }
    }
}
