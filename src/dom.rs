//! DOM implementation.

use std::fmt;
use std::ops::Deref;

use html5ever::rcdom::{self, RcDom};
use selectors::Element;
use selectors::parser::AttrSelector;
use string_cache::{Atom, Namespace};

/// Wrapper around `RcDom`.
pub struct Dom(RcDom);

impl Deref for Dom {
    type Target = RcDom;
    fn deref(&self) -> &RcDom { &self.0 }
}

impl fmt::Debug for Dom {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "Dom(...)")
    }
}

/// Wrapper around `Handle`.
#[derive(Debug)]
pub struct Handle(rcdom::Handle);

impl Deref for Handle {
    type Target = rcdom::Handle;
    fn deref(&self) -> &rcdom::Handle { &self.0 }
}

impl Element for Handle {
    fn parent_element(&self) -> Option<Self> {
        unimplemented!()
    }

    fn first_child_element(&self) -> Option<Self> {
        unimplemented!()
    }

    fn last_child_element(&self) -> Option<Self> {
        unimplemented!()
    }

    fn prev_sibling_element(&self) -> Option<Self> {
        unimplemented!()
    }

    fn next_sibling_element(&self) -> Option<Self> {
        unimplemented!()
    }

    fn is_html_element_in_html_document(&self) -> bool {
        unimplemented!()
    }

    fn get_local_name<'a>(&'a self) -> &'a Atom {
        unimplemented!()
    }

    fn get_namespace<'a>(&'a self) -> &'a Namespace {
        unimplemented!()
    }

    fn get_active_state(&self) -> bool {
        unimplemented!()
    }

    fn get_focus_state(&self) -> bool {
        unimplemented!()
    }

    fn get_hover_state(&self) -> bool {
        unimplemented!()
    }

    fn get_enabled_state(&self) -> bool {
        unimplemented!()
    }

    fn get_disabled_state(&self) -> bool {
        unimplemented!()
    }

    fn get_checked_state(&self) -> bool {
        unimplemented!()
    }

    fn get_intermediate_state(&self) -> bool {
        unimplemented!()
    }

    fn get_id(&self) -> Option<Atom> {
        unimplemented!()
    }

    fn has_class(&self, name: &Atom) -> bool {
        unimplemented!()
    }

    fn match_attr<F>(&self, attr: &AttrSelector, test: F) -> bool where F: Fn(&str) -> bool {
        unimplemented!()
    }

    fn is_empty(&self) -> bool {
        unimplemented!()
    }

    fn is_root(&self) -> bool {
        unimplemented!()
    }

    fn is_link(&self) -> bool {
        unimplemented!()
    }

    fn each_class<F>(&self, callback: F) where F: FnMut(&Atom) {
        unimplemented!()
    }
}
