//! Custom error types for diagnostics
//! Includes re-exported error types from dependencies

use std::{error::Error, fmt::Display};

use cssparser::{BasicParseErrorKind, ParseErrorKind};
use selectors::parser::SelectorParseErrorKind;

/// Error type that is returned when calling `Selector::parse`
#[derive(Debug, Clone)]
pub enum SelectorErrorKind {
    /// A `Token` was not expected
    UnexpectedToken,

    /// End-Of-Line was unexpected
    EndOfLine,

    /// `@` rule is invalid
    InvalidAtRule,

    /// The body of an `@` rule is invalid
    InvalidAtRuleBody,

    /// The qualified rule is invalid
    QualRuleInvalid,

    /// Too many nested blocks
    TooManyNestedBlocks,

    /// Expected a `::` for a pseudoelement
    ExpectedColonOnPseudoElement,

    /// Expected an identity for a pseudoelement
    ExpectedIdentityOnPseudoElement,

    /// A `SelectorParseErrorKind` error that isn't really supposed to happen did
    UnexpectedSelectorParseError(SelectorParseErrorKind),
}

impl From<cssparser::ParseError<SelectorParseErrorKind>> for SelectorErrorKind {
    fn from(original: cssparser::ParseError<SelectorParseErrorKind>) -> Self {
        // NOTE: This could be improved, but I dont
        // exactly know how
        match original.kind {
            ParseErrorKind::Basic(err) => SelectorErrorKind::from(err),
            ParseErrorKind::Custom(err) => SelectorErrorKind::from(err),
        }
    }
}

impl From<BasicParseErrorKind> for SelectorErrorKind {
    fn from(err: BasicParseErrorKind) -> Self {
        match err {
            BasicParseErrorKind::UnexpectedToken => Self::UnexpectedToken,
            BasicParseErrorKind::EndOfInput => Self::EndOfLine,
            BasicParseErrorKind::AtRuleInvalid => Self::InvalidAtRule,
            BasicParseErrorKind::AtRuleBodyInvalid => Self::InvalidAtRuleBody,
            BasicParseErrorKind::QualifiedRuleInvalid => Self::QualRuleInvalid,
            BasicParseErrorKind::TooManyNestedBlocks => Self::TooManyNestedBlocks,
        }
    }
}

impl From<SelectorParseErrorKind> for SelectorErrorKind {
    fn from(err: SelectorParseErrorKind) -> Self {
        match err {
            SelectorParseErrorKind::PseudoElementExpectedColon => {
                Self::ExpectedColonOnPseudoElement
            }
            SelectorParseErrorKind::PseudoElementExpectedIdent => {
                Self::ExpectedIdentityOnPseudoElement
            }
            other => Self::UnexpectedSelectorParseError(other),
        }
    }
}

impl Display for SelectorErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Self::UnexpectedToken => "Token was not expected",
            Self::EndOfLine => "Unexpected EOL",
            Self::InvalidAtRule => "Invalid @-rule",
            Self::InvalidAtRuleBody => "The body of an @-rule was invalid",
            Self::QualRuleInvalid => "The qualified name was invalid",
            Self::TooManyNestedBlocks => "Too many nested blocks",
            Self::ExpectedColonOnPseudoElement => "Missing colon character on pseudoelement",
            Self::ExpectedIdentityOnPseudoElement => "Missing pseudoelement identity",
            Self::UnexpectedSelectorParseError(err) => {
                return write!(
                    f,
                    "Unexpected error occurred. Please report this to the developer\n{err:#?}"
                );
            }
        };

        f.write_str(msg)
    }
}

impl Error for SelectorErrorKind {
    fn description(&self) -> &str {
        match self {
            Self::UnexpectedToken => "Token was not expected",
            Self::EndOfLine => "Unexpected EOL",
            Self::InvalidAtRule => "Invalid @-rule",
            Self::InvalidAtRuleBody => "The body of an @-rule was invalid",
            Self::QualRuleInvalid => "The qualified name was invalid",
            Self::TooManyNestedBlocks => "Too many nested blocks",
            Self::ExpectedColonOnPseudoElement => "Missing colon character on pseudoelement",
            Self::ExpectedIdentityOnPseudoElement => "Missing pseudoelement identity",
            Self::UnexpectedSelectorParseError(_) => "Unexpected error",
        }
    }
}
