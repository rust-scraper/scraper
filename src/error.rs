//! Custom error types for diagnostics
//! Includes re-exported error types from dependencies

mod utils;

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

impl<'a> Display for SelectorErrorKind<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::UnexpectedToken(token) => {
                    format!("Token {:?} was not expected", utils::render_token(token))
                }
                Self::EndOfLine => format!("Unexpected EOL"),
                Self::InvalidAtRule(rule) => format!("Invalid @-rule {:?}", rule),
                Self::InvalidAtRuleBody => format!("The body of an @-rule was invalid"),
                Self::QualRuleInvalid => format!("The qualified name was invalid"),
                Self::ExpectedColonOnPseudoElement(token) => format!(
                    "Expected a ':' token for pseudoelement, got {:?} instead",
                    utils::render_token(token)
                ),
                Self::ExpectedIdentityOnPseudoElement(token) => format!(
                    "Expected identity for pseudoelement, got {:?} instead",
                    utils::render_token(token)
                ),
                Self::UnexpectedSelectorParseError(err) => format!(
                    "Unexpected error occurred. PLEASE REPORT THIS TO THE DEVELOPER\n{:#?}",
                    err
                ),
            }
        )
    }
}
