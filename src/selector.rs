//! CSS selectors.

use std::fmt;
use html5ever::{LocalName, Namespace};
use cssparser;
use selectors::{parser, matching, visitor};

use ElementRef;

/// Wrapper around CSS selectors.
///
/// Represents a "selector group", i.e. a comma-separated list of selectors.
#[derive(Debug, Clone, PartialEq)]
pub struct Selector {
    /// The CSS selectors.
    pub selectors: Vec<parser::Selector<Simple>>,
}

impl Selector {
    /// Parses a CSS selector group.
    ///
    /// No meaningful error can be returned here, due to a limitation of the `selectors` and
    /// `cssparser` crates.
    pub fn parse(selectors: &str) -> Result<Self, ()> {
        let mut parser = cssparser::Parser::new(selectors);
        parser::SelectorList::parse(&Parser, &mut parser).map(|list| Selector { selectors: list.0 })
    }

    /// Returns true if the element matches this selector.
    pub fn matches(&self, element: &ElementRef) -> bool {
        let mut context = matching::MatchingContext::new(matching::MatchingMode::Normal, None);
        self.selectors.iter().any(|s| {
            matching::matches_selector(&s.inner, element, &mut context, &mut |_, _| {})
        })
    }
}

/// An implementation of `Parser` for `selectors`
struct Parser;
impl parser::Parser for Parser {
    type Impl = Simple;
}


/// A simple implementation of `SelectorImpl` with no pseudo-classes or pseudo-elements.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Simple;

impl parser::SelectorImpl for Simple {
    type AttrValue = String;
    type Identifier = LocalName;
    type ClassName = LocalName;
    type LocalName = LocalName;
    type NamespacePrefix = LocalName;
    type NamespaceUrl = Namespace;
    type BorrowedNamespaceUrl = Namespace;
    type BorrowedLocalName = LocalName;

    type NonTSPseudoClass = NonTSPseudoClass;
    type PseudoElement = PseudoElement;
}

/// Non Tree-Structural Pseudo-Class.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NonTSPseudoClass {}

impl parser::SelectorMethods for NonTSPseudoClass {
    type Impl = Simple;

    fn visit<V>(&self, _visitor: &mut V) -> bool
    where
        V: visitor::SelectorVisitor<Impl = Self::Impl>,
    {
        true
    }
}

impl cssparser::ToCss for NonTSPseudoClass {
    fn to_css<W>(&self, dest: &mut W) -> fmt::Result
    where
        W: fmt::Write,
    {
        dest.write_str("")
    }
}

/// CSS Pseudo-Element
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PseudoElement {}

impl parser::PseudoElement for PseudoElement {
    type Impl = Simple;
}

impl cssparser::ToCss for PseudoElement {
    fn to_css<W>(&self, dest: &mut W) -> fmt::Result
    where
        W: fmt::Write,
    {
        dest.write_str("")
    }
}
