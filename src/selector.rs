//! CSS selectors.

use cssparser::Parser;
use selectors::parser::{self, ParserContext};

/// Wrapper around CSS selectors.
#[derive(Debug, Clone, PartialEq)]
pub struct Selector {
    /// The CSS selectors.
    pub selectors: Vec<parser::Selector>,
}

impl Selector {
    /// Parses a CSS selector group.
    pub fn parse(selectors: &str) -> Result<Self, ()> {
        let mut parser = Parser::new(selectors);
        let context = ParserContext::new();
        let selectors = try!(parser::parse_selector_list(&context, &mut parser));
        Ok(Selector { selectors: selectors })
    }
}
