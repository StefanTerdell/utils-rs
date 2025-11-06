use std::fmt::{Debug, Display};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "schemars", derive(::schemars::JsonSchema))]
pub enum Comparison {
    Less,
    LessOrEqual,
    Equal,
    Unequal,
    GreaterOrEqual,
    Greater,
}

impl Display for Comparison {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Comparison::Less => "less than",
            Comparison::LessOrEqual => "less than or equal to",
            Comparison::Equal => "equal to",
            Comparison::Unequal => "unequal to",
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
            Comparison::Unequal => a != b,
            Comparison::GreaterOrEqual => a >= b,
            Comparison::Greater => a > b,
        }
    }
}
