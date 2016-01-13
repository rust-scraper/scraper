//! HTML comments.

use std::fmt;
use std::ops::Deref;

use tendril::StrTendril;

/// An HTML comment.
pub struct Comment {
    comment: StrTendril,
}

impl Deref for Comment {
    type Target = str;

    fn deref(&self) -> &str {
        self.comment.deref()
    }
}

impl fmt::Debug for Comment {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "<!-- {:?} -->", self.deref())
    }
}
