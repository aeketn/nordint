// version: 0.1.0
// author:  Erik Nordin
// created: 07/14/2018
// updated: 07/14/2018
// contact: aeketn@gmail.com

mod biguint;
pub use biguint::BigUint;

use std::error::Error;
use std::fmt;

// Error type design taken from num-bigint on crates.io
// https://crates.io/crates/num-bigint
// which is, in turn, modeled after the std::num::ParseInterror
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseBigIntError {
    kind: BigIntErrorKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum BigIntErrorKind {
    Empty,
    InvalidDigit,
}

impl ParseBigIntError {
    fn __description(&self) -> &str {
        use BigIntErrorKind::*;
        match self.kind {
            Empty => "cannot parse integer from empty string",
            InvalidDigit => "invalid digit found in string",
        }
    }

    fn empty() -> Self {
        ParseBigIntError {
            kind: BigIntErrorKind::Empty,
        }
    }

    fn invalid() -> Self {
        ParseBigIntError {
            kind: BigIntErrorKind::InvalidDigit,
        }
    }
}

impl fmt::Display for ParseBigIntError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.__description().fmt(f)
    }
}

impl Error for ParseBigIntError {
    fn description(&self) -> &str {
        self.__description()
    }
}
