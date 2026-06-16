#[macro_export]
macro_rules! literal {
($(#[$attr:meta])* $name:ident($type:ty) = $value:expr) => {
    #[derive(
        ::core::fmt::Debug, ::core::clone::Clone, ::serde::Serialize, ::serde::Deserialize,
    )]
    $(#[$attr])*
    pub struct $name($type);

    impl<T: PartialEq<$type>> PartialEq<T> for $name {
        fn eq(&self, other: &T) -> bool {
            other.eq(&self.0)
        }
    }

    impl ::schemars::JsonSchema for $name {
        fn schema_name() -> std::borrow::Cow<'static, str> {
            stringify!($name).into()
        }

        fn json_schema(_: &mut ::schemars::SchemaGenerator) -> ::schemars::Schema {
            ::schemars::json_schema!({ "const": $value })
        }
    }

    impl ::std::fmt::Display for $name {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_fmt(format_args!("{}", &self.0))
        }
    }

    impl AsRef<$type> for $name {
        fn as_ref(&self) -> &$type {
            &self.0
        }
    }

    impl TryFrom<$type> for $name {
        type Error = String;

        fn try_from(value: $type) -> ::std::result::Result<Self, Self::Error> {
            if value == $value {
                Ok(Self(value))
            } else {
                Err(format!(
                    "Value must be exactly {}",
                    ::serde_json::json!($value)
                ))
            }
        }
    }

    impl From<$name> for $type {
        fn from(value: $name) -> Self {
            value.0
        }
    }
};
}

#[macro_export]
macro_rules! literal_string {
    ($name:ident = $value:expr) => {
        $crate::literal!(
            #[serde(try_from = "String")]
            $name(String) = $value
        );

        impl Default for $name {
            fn default() -> Self {
                Self($value.to_string())
            }
        }
    };
}

#[macro_export]
macro_rules! literal_u32 {
    ($name:ident = $value:expr) => {
        $crate::literal!(
            #[serde(try_from = "u32")]
            $name(u32) = $value
        );

        impl Default for $name {
            fn default() -> Self {
                Self($value)
            }
        }
    };
}

#[macro_export]
macro_rules! literal_i32 {
    ($name:ident = $value:expr) => {
        $crate::literal!(
            #[serde(try_from = "i32")]
            $name(i32) = $value
        );

        impl Default for $name {
            fn default() -> Self {
                Self($value)
            }
        }
    };
}

#[macro_export]
macro_rules! literal_f32 {
    ($name:ident = $value:expr) => {
        $crate::literal!(
            #[serde(try_from = "f32")]
            $name(f32) = $value
        );

        impl Default for $name {
            fn default() -> Self {
                Self($value)
            }
        }
    };
}

#[macro_export]
macro_rules! literal_bool {
    ($name:ident = $value:expr) => {
        $crate::literal!(
            #[serde(try_from = "bool")]
            $name(bool) = $value
        );

        impl Default for $name {
            fn default() -> Self {
                Self($value)
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use schemars::{json_schema, schema_for};
    use serde_json::json;

    crate::literal_string!(Xbox = "xbox");

    #[test]
    fn literal_string_should_deserialize_ok_from_correct_value() {
        let ok = serde_json::from_value::<Xbox>(json!("xbox")).unwrap();

        assert_eq!(ok, "xbox");
    }

    #[test]
    fn literal_string_should_deserialize_err_from_correct_type_but_wrong_value() {
        let err = serde_json::from_value::<Xbox>(json!("ps4")).unwrap_err();

        assert_eq!(err.to_string(), "Value must be exactly \"xbox\"");
    }

    #[test]
    fn literal_string_should_produce_correct_json_schema() {
        let schema = schema_for!(Xbox);

        assert_eq!(
            schema,
            json_schema!({
                "$schema": "https://json-schema.org/draft/2020-12/schema",
                "title": "Xbox",
                "const": "xbox"
            })
        );
    }

    literal_u32!(OneTwoThree = 123);

    #[test]
    fn literal_u32_should_deserialize_ok_from_correct_value() {
        let ok = serde_json::from_value::<OneTwoThree>(json!(123)).unwrap();

        assert_eq!(ok, 123);
    }

    #[test]
    fn literal_u32_should_deserialize_err_from_correct_type_but_wrong_value() {
        let err = serde_json::from_value::<OneTwoThree>(json!(321)).unwrap_err();

        assert_eq!(err.to_string(), "Value must be exactly 123");
    }

    #[test]
    fn literal_u32_should_produce_correct_json_schema() {
        let schema = schema_for!(OneTwoThree);

        assert_eq!(
            schema,
            json_schema!({
                "$schema": "https://json-schema.org/draft/2020-12/schema",
                "title": "OneTwoThree",
                "const": 123
            })
        );
    }
}
