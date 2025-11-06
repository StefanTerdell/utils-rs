use crate::{
    comparator::{Comparison, ExpectedComparisonError},
    with_len::WithLen,
};
use core::error::Error;
use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "schemars", derive(::schemars::JsonSchema))]
pub struct ExpectedLenError {
    pub comparison: Comparison,
    pub comparator: usize,
    pub actual: usize,
}

impl From<ExpectedComparisonError<usize>> for ExpectedLenError {
    fn from(
        ExpectedComparisonError {
            comparison,
            comparator,
            actual,
        }: ExpectedComparisonError<usize>,
    ) -> Self {
        ExpectedLenError {
            comparison,
            comparator,
            actual,
        }
    }
}

impl Error for ExpectedLenError {}

impl Display for ExpectedLenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Expected length to be {} {}, but it was {}",
            self.comparison, self.comparator, self.actual
        ))
    }
}

pub trait ExpectLen: WithLen + Sized {
    fn expect_len_lt(self, length: usize) -> Result<Self, ExpectedLenError>;
    fn expect_len_lte(self, length: usize) -> Result<Self, ExpectedLenError>;
    fn expect_len_eq(self, length: usize) -> Result<Self, ExpectedLenError>;
    fn expect_len_gte(self, length: usize) -> Result<Self, ExpectedLenError>;
    fn expect_len_gt(self, length: usize) -> Result<Self, ExpectedLenError>;
}

impl<T: WithLen + Sized> ExpectLen for T {
    fn expect_len_lt(self, length: usize) -> Result<Self, ExpectedLenError> {
        Comparison::Less
            .expect_comparison(self.len(), length)
            .map_err(ExpectedLenError::from)
            .map(|_| self)
    }

    fn expect_len_lte(self, length: usize) -> Result<Self, ExpectedLenError> {
        Comparison::LessOrEqual
            .expect_comparison(self.len(), length)
            .map_err(ExpectedLenError::from)
            .map(|_| self)
    }

    fn expect_len_eq(self, length: usize) -> Result<Self, ExpectedLenError> {
        Comparison::Equal
            .expect_comparison(self.len(), length)
            .map_err(ExpectedLenError::from)
            .map(|_| self)
    }

    fn expect_len_gte(self, length: usize) -> Result<Self, ExpectedLenError> {
        Comparison::GreaterOrEqual
            .expect_comparison(self.len(), length)
            .map_err(ExpectedLenError::from)
            .map(|_| self)
    }

    fn expect_len_gt(self, length: usize) -> Result<Self, ExpectedLenError> {
        Comparison::Greater
            .expect_comparison(self.len(), length)
            .map_err(ExpectedLenError::from)
            .map(|_| self)
    }
}
