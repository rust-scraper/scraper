//! CSS selectors.

use cssparser::Parser;
use selectors::{matching, Element};
use selectors::parser::{self, ParserContext};

/// Wrapper around CSS selectors.
///
/// Represents a "selector group", i.e. a comma-separated list of selectors.
///
/// # Examples
///
/// ```
/// use scraper::Selector;
/// let selector = Selector::parse("h1.foo, h2.foo").unwrap();
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Selector {
    /// The CSS selectors.
    pub selectors: Vec<parser::Selector>,
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
    ///
    /// The `NodeRef` wrapper implements the `Element` trait required here.
    ///
    /// # Panics
    ///
    /// Panics if a `NodeRef` does not reference an element.
    pub fn matches<E: Element>(&self, node: &E) -> bool {
        matching::matches(&self.selectors, node, None)
    }
}
