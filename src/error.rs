//! Custom error types for diagnostics
//! Includes re-exported error types from dependencies

use std::fmt::Display;

use cssparser::{BasicParseErrorKind, ParseErrorKind, Token};
use selectors::parser::SelectorParseErrorKind;

/// Error type that is returned when calling `Selector::parse`
#[derive(Debug, Clone)]
pub enum SelectorErrorKind<'a> {
    /// A `Token` was not expected
    UnexpectedToken(Token<'a>),

    /// End-Of-Line was unexpected
    EndOfLine,

    /// `@` rule is invalid
    InvalidAtRule(String),

    /// The body of an `@` rule is invalid
    InvalidAtRuleBody,

    /// The qualified rule is invalid
    QualRuleInvalid,

    /// Expected a `::` for a pseudoelement
    ExpectedColonOnPseudoElement(Token<'a>),

    /// Expected an identity for a pseudoelement
    ExpectedIdentityOnPseudoElement(Token<'a>),

    /// A `SelectorParseErrorKind` error that isn't really supposed to happen did
    UnexpectedSelectorParseError(SelectorParseErrorKind<'a>),
}

impl<'a> From<cssparser::ParseError<'a, SelectorParseErrorKind<'a>>> for SelectorErrorKind<'a> {
    fn from(original: cssparser::ParseError<'a, SelectorParseErrorKind<'a>>) -> Self {
        // To anyone who dares to read this code
        // I commend you, i guess. I'm so sorry
        // for the abomination that is casting
        // stuff into this one enum

        match original.kind {
            ParseErrorKind::Basic(err) => match err {
                BasicParseErrorKind::UnexpectedToken(token) => Self::UnexpectedToken(token),
                BasicParseErrorKind::EndOfInput => Self::EndOfLine,
                BasicParseErrorKind::AtRuleInvalid(rule) => {
                    Self::InvalidAtRule(rule.clone().to_string())
                }
                BasicParseErrorKind::AtRuleBodyInvalid => Self::InvalidAtRuleBody,
                BasicParseErrorKind::QualifiedRuleInvalid => Self::QualRuleInvalid,
            },
            ParseErrorKind::Custom(err) => match err {
                SelectorParseErrorKind::PseudoElementExpectedColon(token) => {
                    Self::ExpectedColonOnPseudoElement(token)
                }
                SelectorParseErrorKind::PseudoElementExpectedIdent(token) => {
                    Self::ExpectedIdentityOnPseudoElement(token)
                }
                other => Self::UnexpectedSelectorParseError(other),
            },
        }
    }
}
