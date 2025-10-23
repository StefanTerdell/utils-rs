use crate::as_str::AsStr;

impl AsStr for bool {
    fn as_str(&self) -> &str {
        match self {
            true => "true",
            false => "false",
        }
    }
}
