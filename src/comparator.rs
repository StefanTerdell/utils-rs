use core::error::Error;
use std::fmt::{Debug, Display};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "schemars", derive(::schemars::JsonSchema))]
pub enum Comparison {
    Less,
    LessOrEqual,
    Equal,
    GreaterOrEqual,
    Greater,
}

impl Display for Comparison {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Comparison::Less => "less than",
            Comparison::LessOrEqual => "less than or equal to",
            Comparison::Equal => "equal to",
            Comparison::GreaterOrEqual => "greater than or equal to",
            Comparison::Greater => "greater than",
        })
    }
}

impl Comparison {
    pub fn compare<T: PartialOrd + PartialEq>(&self, a: &T, b: &T) -> bool {
        match self {
            Comparison::Less => a < b,
            Comparison::LessOrEqual => a <= b,
            Comparison::Equal => a == b,
            Comparison::GreaterOrEqual => a >= b,
            Comparison::Greater => a > b,
        }
    }

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
