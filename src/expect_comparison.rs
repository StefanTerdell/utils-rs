use crate::comparator::{Comparison, ExpectedComparisonError};

pub trait ExpectComparison: PartialOrd + PartialEq + Sized {
    fn expect_lt(self, other: Self) -> Result<Self, ExpectedComparisonError<Self>>;
    fn expect_lte(self, other: Self) -> Result<Self, ExpectedComparisonError<Self>>;
    fn expect_eq(self, other: Self) -> Result<Self, ExpectedComparisonError<Self>>;
    fn expect_gte(self, other: Self) -> Result<Self, ExpectedComparisonError<Self>>;
    fn expect_gt(self, other: Self) -> Result<Self, ExpectedComparisonError<Self>>;
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

    fn expect_gte(self, other: Self) -> Result<Self, ExpectedComparisonError<Self>> {
        Comparison::GreaterOrEqual.expect_comparison(self, other)
    }

    fn expect_gt(self, other: Self) -> Result<Self, ExpectedComparisonError<Self>> {
        Comparison::Greater.expect_comparison(self, other)
    }
}
