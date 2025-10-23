use crate::as_bool::AsBool;

impl AsBool for str {
    fn as_bool(&self) -> bool {
        matches!(
            self,
            "true"
                | "TRUE"
                | "True"
                | "truE"
                | "trUe"
                | "trUE"
                | "tRue"
                | "tRuE"
                | "tRUe"
                | "tRUE"
                | "TruE"
                | "TrUe"
                | "TrUE"
                | "TRue"
                | "TRuE"
                | "TRUe"
        )
    }
}
