//! HTML text.

use std::fmt;
use std::ops::Deref;

use tendril::StrTendril;

/// HTML text.
pub struct Text {
    text: StrTendril,
}

impl Deref for Text {
    type Target = str;

    fn deref(&self) -> &str {
        self.text.deref()
    }
}

impl fmt::Debug for Text {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{:?}", self.deref())
    }
}
