use crate::as_bool::AsBool;
use crate::from_str::FromStr;

impl FromStr for bool {
    fn from_str(str: &str) -> Self {
        str.as_bool()
    }
}
