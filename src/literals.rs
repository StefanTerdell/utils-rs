#[macro_export]
macro_rules! literal {
    ($(#[$attr:meta])* $name:ident($type:tt) = $value:literal) => {
        literal!($(#[$attr])* $name($type => $type) = $value);
    };

    ($(#[$attr:meta])* $name:ident($serde_from:tt => $type:tt) = $value:literal) => {
        literal!(@ $(#[$attr])* $name($serde_from => $type) = $value);

        impl $name {
            pub const VALUE: $type = $value;
        }

        impl<T: PartialEq<$type>> PartialEq<T> for $name {
            fn eq(&self, other: &T) -> bool {
                other.eq(&Self::VALUE)
            }
        }
    };

    ($(#[$attr:meta])* $name:ident(&$type:tt) = $value:literal) => {
        literal!($(#[$attr])* $name($type => $type) = $value);
    };

    ($(#[$attr:meta])* $name:ident($serde_from:tt => &$type:tt) = $value:literal) => {
        literal!(@ $(#[$attr])* $name($serde_from => $type) = $value);

        impl $name {
            pub const VALUE: &$type = $value;
        }

        impl<T: for<'a> PartialEq<&'a $type>> PartialEq<T> for $name {
            fn eq(&self, other: &T) -> bool {
                other.eq(&Self::VALUE)
            }
        }
    };

    (@ $(#[$attr:meta])*  $name:ident($serde_from:tt => $type:tt) = $value:literal) => {
        #[derive(Clone, Copy)]
        $(#[$attr])*
        pub struct $name;

        impl AsRef<$type> for $name {
            fn as_ref(&self) -> &$type {
                &Self::VALUE
            }
        }

        impl ::std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_tuple($name::NAME).field(&$name::VALUE).finish()
            }
        }

        impl $name {
            const NAME: &'static str = stringify!($name);
        }

        impl Default for $name {
            fn default() -> Self {
                Self
            }
        }

        impl ::serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
            {
                serializer.serialize_newtype_struct($name::NAME, &$name::VALUE)
            }
        }

        impl<'de> ::serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::serde::Deserializer<'de>,
            {
                <$serde_from as ::serde::Deserialize<'de>>::deserialize(deserializer).and_then(
                    |value| {
                        if value == $name::VALUE {
                            Ok($name)
                        } else {
                            Err(::serde::de::Error::custom(concat!(
                                "Value must be exactly ",
                                stringify!($value)
                            )))
                        }
                    },
                )
            }
        }

        impl ::schemars::JsonSchema for $name {
            fn schema_name() -> ::std::borrow::Cow<'static, str> {
                $name::NAME.into()
            }

            fn json_schema(_: &mut ::schemars::SchemaGenerator) -> ::schemars::Schema {
                ::schemars::json_schema!({
                    "const": $name::VALUE
                })
            }
        }

        impl Eq for $name {}
    };
}

#[macro_export]
macro_rules! literal_str {
    ($(#[$attr:meta])* $name:ident = $value:literal) => {
        $crate::literal!($(#[$attr])* $name(String => &str) = $value);
    };
}

macro_rules! literal_scalars {
    ($d:tt $($type:ident)*) => {
        $(
            ::paste::paste! {
                #[macro_export]
                macro_rules! [<literal_ $type>] {
                    ($d (#[$d attr:meta])* $d name:ident = $d value:literal) => {
                        $crate::literal!($d (#[$d attr])* $d name ($type) = $d value);
                    }
                }
            }
        )*
    }
}

literal_scalars!($ u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize f32 f64 bool);

#[cfg(test)]
mod tests {
    #[test]
    fn literal_str_should_work() {
        literal_str!(
            #[doc = "metametameta"]
            MyStr = "my-value"
        );

        assert_eq!(
            serde_json::from_str::<MyStr>("\"my-value\"").unwrap(),
            "my-value"
        );

        assert_eq!(MyStr, MyStr);
    }

    #[test]
    fn literal_u8_should_work() {
        literal_u8!(Two = 2);
        literal_u8!(
            #[doc = "lololol"]
            AlsoTwo = 2
        );

        assert_eq!(Two, 2);
        assert_eq!(Two, Two);
        assert_eq!(Two, AlsoTwo);
        assert_eq!(AlsoTwo, Two);
        assert_ne!(Two, 3);
    }
    use schemars::{json_schema, schema_for};
    use serde_json::json;

    crate::literal_str!(Xbox = "xbox");

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
