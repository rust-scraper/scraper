//! CSS selectors.

use std::fmt;

use smallvec::SmallVec;

use html5ever::{LocalName, Namespace};
use cssparser;
use selectors::{matching, parser, visitor};
use selectors::parser::SelectorParseErrorKind;

use ElementRef;

/// Wrapper around CSS selectors.
///
/// Represents a "selector group", i.e. a comma-separated list of selectors.
#[derive(Debug, Clone, PartialEq)]
pub struct Selector {
    /// The CSS selectors.
    pub selectors: SmallVec<[parser::Selector<Simple>; 1]>,
}

impl Selector {
    /// Parses a CSS selector group.

    pub fn parse<'t, 'i>(
        selectors: &'i str,
    ) -> Result<Self, cssparser::ParseError<'i, SelectorParseErrorKind<'i>>> {
        let mut parser_input = cssparser::ParserInput::new(selectors);
        let mut parser = cssparser::Parser::new(&mut parser_input);
        parser::SelectorList::parse(&Parser, &mut parser).map(|list| Selector { selectors: list.0 })
    }

    /// Returns true if the element matches this selector.
    pub fn matches(&self, element: &ElementRef) -> bool {
        let mut context = matching::MatchingContext::new(
            matching::MatchingMode::Normal,
            None,
            None,
            matching::QuirksMode::NoQuirks,
        );
        self.selectors
            .iter()
            .any(|s| matching::matches_selector(&s, 0, None, element, &mut context, &mut |_, _| {}))
    }
}

/// An implementation of `Parser` for `selectors`
struct Parser;
impl<'i> parser::Parser<'i> for Parser {
    type Impl = Simple;
    type Error = SelectorParseErrorKind<'i>;
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

    // see: https://github.com/servo/servo/pull/19747#issuecomment-357106065
    type ExtraMatchingData = String;
}

/// Non Tree-Structural Pseudo-Class.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NonTSPseudoClass {}

impl parser::NonTSPseudoClass for NonTSPseudoClass {
    type Impl = Simple;

    fn is_active_or_hover(&self) -> bool {
        false
    }
}

impl parser::Visit for NonTSPseudoClass {
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
