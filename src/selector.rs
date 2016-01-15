//! CSS selectors.

use cssparser::Parser;
use selectors::matching;
use selectors::parser::{self, ParserContext};

use node_ref::NodeRef;

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

    /// Returns true if the referenced element matches this selector.
    ///
    /// # Panics
    ///
    /// Panics if referenced node is not an element.
    pub fn matches(&self, node: NodeRef) -> bool {
        matching::matches(&self.selectors, &node, None)
    }
}
