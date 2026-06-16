#[macro_export]
macro_rules! literal {
    ($(#[$attr:meta])* $name:ident($type:ty) = $value:expr) => {
        #[derive(::core::fmt::Debug, ::core::clone::Clone, ::serde::Serialize)]
        $(#[$attr])*
        pub struct $name($type);

        impl<'de> ::serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, D::Error>
            where
                D: ::serde::Deserializer<'de>,
            {
                let value = <$type as ::serde::Deserialize<'de>>::deserialize(deserializer)?;

                Self::try_from(value).map_err(::serde::de::Error::custom)
            }
        }

        impl<T: PartialEq<$type>> PartialEq<T> for $name {
            fn eq(&self, other: &T) -> bool {
                other.eq(&self.0)
            }
        }

        impl ::schemars::JsonSchema for $name {
            fn schema_name() -> ::std::borrow::Cow<'static, str> {
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
                    Err(format!("Value must be exactly {:?}", $value))
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
        $crate::literal!($name(String) = $value);

        impl Default for $name {
            fn default() -> Self {
                Self($value.to_string())
            }
        }
    };
}

macro_rules! literal_scalars {
    ($d:tt $($type:ident)*) => {
        $(
            ::paste::paste! {
                #[macro_export]
                macro_rules! [<literal_ $type>] {
                    ($d name:ident = $d value:expr) => {
                        $crate::literal!($d name($type) = $d value);

                        impl Default for $d name {
                            fn default() -> Self {
                                Self($d value)
                            }
                        }
                    };
                }
            }
        )*
    };
}

literal_scalars!($ u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize f32 f64 bool);

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
