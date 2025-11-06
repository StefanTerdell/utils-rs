use crate::comparator::Comparison;
use core::error::Error;
use std::fmt::{Debug, Display};

pub trait ExpectComparison: PartialOrd + PartialEq + Sized {
    fn expect_lt(self, other: Self) -> Result<Self, ExpectedComparisonError<Self>>;
    fn expect_lte(self, other: Self) -> Result<Self, ExpectedComparisonError<Self>>;
    fn expect_eq(self, other: Self) -> Result<Self, ExpectedComparisonError<Self>>;
    fn expect_ne(self, other: Self) -> Result<Self, ExpectedComparisonError<Self>>;
    fn expect_gte(self, other: Self) -> Result<Self, ExpectedComparisonError<Self>>;
    fn expect_gt(self, other: Self) -> Result<Self, ExpectedComparisonError<Self>>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "schemars", derive(::schemars::JsonSchema))]
pub struct ExpectedComparisonError<T> {
    pub comparison: Comparison,
    pub comparator: T,
    pub actual: T,
}

impl<T: Debug> Error for ExpectedComparisonError<T> {}

impl<T: Debug> Display for ExpectedComparisonError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Expected value to be {} {:?}, but it was {:?}",
            self.comparison, self.comparator, self.actual
        ))
    }
}

impl Comparison {
    pub fn expect_comparison<T: PartialOrd + PartialEq>(
        &self,
        actual: T,
        comparator: T,
    ) -> Result<T, ExpectedComparisonError<T>> {
        if self.compare(&actual, &comparator) {
            Ok(actual)
        } else {
            Err(ExpectedComparisonError {
                comparison: *self,
                comparator,
                actual,
            })
        }
    }
}

impl<T: PartialOrd + PartialEq + Sized> ExpectComparison for T {
    fn expect_lt(self, other: Self) -> Result<Self, ExpectedComparisonError<Self>> {
        Comparison::Less.expect_comparison(self, other)
    }

    fn expect_lte(self, other: Self) -> Result<Self, ExpectedComparisonError<Self>> {
        Comparison::LessOrEqual.expect_comparison(self, other)
    }

    fn expect_eq(self, other: Self) -> Result<Self, ExpectedComparisonError<Self>> {
        Comparison::Equal.expect_comparison(self, other)
    }

    fn expect_ne(self, other: Self) -> Result<Self, ExpectedComparisonError<Self>> {
        Comparison::Unequal.expect_comparison(self, other)
    }

    fn expect_gte(self, other: Self) -> Result<Self, ExpectedComparisonError<Self>> {
        Comparison::GreaterOrEqual.expect_comparison(self, other)
    }

    fn expect_gt(self, other: Self) -> Result<Self, ExpectedComparisonError<Self>> {
        Comparison::Greater.expect_comparison(self, other)
    }
}
