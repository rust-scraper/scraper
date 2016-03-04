//! CSS selectors.

use cssparser::Parser;
use selectors::matching;
use selectors::parser::{self, ParserContext, SelectorImpl};

use element_ref::ElementRef;

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
        let mut parser = Parser::new(selectors);
        let context = ParserContext::new();
        let selectors = try!(parser::parse_selector_list(&context, &mut parser));
        Ok(Selector { selectors: selectors })
    }

    /// Returns true if the element matches this selector.
    pub fn matches(&self, element: &ElementRef) -> bool {
        matching::matches(&self.selectors, element, None)
    }
}

/// A simple implementation of `SelectorImpl` with no pseudo-classes or pseudo-elements.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Simple;
impl SelectorImpl for Simple {
    type NonTSPseudoClass = ();
    type PseudoElement = ();
}
