use selectors::Element;
use selectors::attr::{AttrSelectorOperation, NamespaceConstraint};
use html5ever::{Namespace, LocalName};
use selectors::matching;

use super::ElementRef;
use selector::{Simple, NonTSPseudoClass, PseudoElement};

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

    fn get_local_name(&self) -> &LocalName {
        &self.value().name.local
    }

    fn get_namespace(&self) -> &Namespace {
        &self.value().name.ns
    }

    fn match_non_ts_pseudo_class<F>(
        &self,
        _pc: &NonTSPseudoClass,
        _context: &mut matching::MatchingContext,
        _flags_setter: &mut F,
    ) -> bool {
        false
    }

    fn get_id(&self) -> Option<LocalName> {
        self.value().id.clone()
    }

    fn has_class(&self, name: &LocalName) -> bool {
        self.value().classes.contains(name)
    }

    fn is_empty(&self) -> bool {
        !self.children()
            .any(|child| child.value().is_element() || child.value().is_text())
    }

    fn is_root(&self) -> bool {
        self.parent()
            .map_or(false, |parent| parent.value().is_document())
    }

    fn attr_matches(
        &self,
        ns: &NamespaceConstraint<&Namespace>,
        local_name: &LocalName,
        operation: &AttrSelectorOperation<&String>,
    ) -> bool {
        self.value().attrs.iter().any(|(key, value)| {
            !matches!(*ns, NamespaceConstraint::Specific(url) if *url != key.ns) &&
                *local_name == key.local && operation.eval_str(value)
        })
    }

    fn match_pseudo_element(
        &self,
        _pe: &PseudoElement,
        _context: &mut matching::MatchingContext,
    ) -> bool {
        false
    }
}
