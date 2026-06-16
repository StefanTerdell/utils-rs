//! Contains the `Secret<T>` struct, designed to wrap values that should never appear formatted with Display or Debug in logs or other output.

use crate::prelude::{AsClone, AsCopy};

/// Wraps a value that should never appear formatted with Display or Debug in logs or other output.
///
/// `Secret<T>` does implement `Debug`, but only outputs the inner type name, ie. `"Secret<String>"`;
///
/// Forwards the following core traits from its inner type:
/// - `Clone`
/// - `Copy`
/// - `PartialeEq`
/// - `Eq`
/// - `PartialOrd`
/// - `Ord`
///
/// ## Feature-flagged trait implementations
///
/// | Feature    | Trait         |
/// | ---------- | ------------- |
/// | `core`     | `Debug` (1)   |
/// | `core`     | `Clone` (2)   |
/// | `core`     | `Copy` (3)    |
/// | `core`     | `PartialEq`   |
/// | `core`     | `Eq`          |
/// | `core`     | `PartialOrd`  |
/// | `core`     | `Ord`         |
/// | `core`     | `Hash`        |
/// | `serde`    | `Serialize`   |
/// | `serde`    | `Deserialize` |
/// | `schemars` | `JsonSchema`  |
/// | `sqlx`     | `Type`        |
///
/// All trait implementations besides `Debug` are forwarded from the inner type.
///
/// 1. `Debug::fmt` only produces the name of the wrapped type, ie. `"Secret<&str>"`
/// 2. `T::Clone` enables the `expose_clone(&self) -> T` method of `Secret<T>`
/// 3. `T::Copy` enables the `expose_copy(&self) -> T` method of `Secret<T>`
pub struct Secret<T> {
    value: T,
    serialize_redacted: bool,
}

impl<T> Secret<T> {
    pub fn new(value: T) -> Self {
        let serialize_redacted = {
            #[cfg(feature = "secret-serialize-redacted")]
            {
                true
            }
            #[cfg(not(feature = "secret-serialize-redacted"))]
            {
                false
            }
        };

        Self {
            value,
            serialize_redacted,
        }
    }

    pub fn set_serialize_redacted(&mut self, serialize_redacted: bool) {
        self.serialize_redacted = serialize_redacted;
    }

    pub fn with_serialize_redacted(self, serialize_redacted: bool) -> Self {
        Self {
            value: self.value,
            serialize_redacted,
        }
    }

    pub fn expose(self) -> T {
        self.value
    }

    pub fn expose_ref(&self) -> &T {
        &self.value
    }

    pub fn expose_as_ref<R: ?Sized>(&self) -> &R
    where
        T: AsRef<R>,
    {
        self.expose_ref().as_ref()
    }
}

impl<T: Clone> Secret<T> {
    pub fn expose_clone(&self) -> T {
        self.value.clone()
    }

    pub fn expose_as_clone<R>(&self) -> R
    where
        for<'a> &'a T: AsClone<R>,
    {
        (&self.value).as_clone()
    }
}

impl<T: Copy> Secret<T> {
    pub fn expose_copy(&self) -> T {
        self.value
    }

    pub fn expose_as_copy<R>(&self) -> R
    where
        for<'a> &'a T: AsCopy<R>,
    {
        (&self.value).as_copy()
    }
}

impl<T> From<T> for Secret<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl<T: Clone> Clone for Secret<T> {
    fn clone(&self) -> Self {
        Self::new(self.value.clone())
    }
}

impl<T: Copy> Copy for Secret<T> {}

impl<T: PartialEq> PartialEq for Secret<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value.eq(&other.value)
    }
}

impl<T: Eq> Eq for Secret<T> {}

impl<T: PartialOrd> PartialOrd for Secret<T> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl<T: Ord> Ord for Secret<T> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.value.cmp(&other.value)
    }
}

impl<T> core::fmt::Debug for Secret<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!("Secret<{}>", core::any::type_name::<T>()))
    }
}

impl<T: core::hash::Hash> core::hash::Hash for Secret<T> {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state)
    }
}

#[cfg(feature = "serde")]
mod serde {
    use super::Secret;

    impl<T: serde::Serialize> serde::Serialize for Secret<T> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            if self.serialize_redacted {
                format!("Secret<{}>", core::any::type_name::<T>()).serialize(serializer)
            } else {
                self.value.serialize(serializer)
            }
        }
    }

    #[cfg(feature = "serde")]
    impl<'de, T: serde::Deserialize<'de>> serde::Deserialize<'de> for Secret<T> {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            T::deserialize(deserializer).map(Secret::new)
        }
    }
}

#[cfg(feature = "schemars")]
mod schemars {
    use super::Secret;
    extern crate alloc;

    impl<T: schemars::JsonSchema> schemars::JsonSchema for Secret<T> {
        fn schema_id() -> alloc::borrow::Cow<'static, str> {
            T::schema_id()
        }

        fn schema_name() -> alloc::borrow::Cow<'static, str> {
            T::schema_name()
        }

        fn inline_schema() -> bool {
            T::inline_schema()
        }

        fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
            #[cfg(feature = "secret-serialize-redacted")]
            {
                schemars::json_schema!({
                    "anyOf": [
                        {
                            "const": format!("Secret<{}>", core::any::type_name::<T>())
                        },
                        T::json_schema(generator)
                    ]
                })
            }
            #[cfg(not(feature = "secret-serialize-redacted"))]
            {
                T::json_schema(generator)
            }
        }
    }
}

#[cfg(feature = "sqlx")]
mod sqlx_impls {
    impl<DB: sqlx::Database, T: sqlx::Type<DB>> sqlx::Type<DB> for super::Secret<T> {
        fn type_info() -> DB::TypeInfo {
            T::type_info()
        }

        fn compatible(ty: &DB::TypeInfo) -> bool {
            T::compatible(ty)
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn debug_should_output_type_only() {
        let secret = Secret::new("Hello, world!");

        assert_eq!(format!("{secret:#?}"), "Secret<&str>");
    }

    #[test]
    fn serialize_redacted_should_output_type_only() {
        let secret = Secret::new("Hello, world!").with_serialize_redacted(true);

        assert_eq!(serde_json::to_string(&secret).unwrap(), "\"Secret<&str>\"");
    }

    #[test]
    fn serialize_unredacted_should_output_full_value() {
        let secret = Secret::new("Hello, world!").with_serialize_redacted(false);

        assert_eq!(serde_json::to_string(&secret).unwrap(), "\"Hello, world!\"");
    }

    #[test]
    fn should_be_able_to_expose_as_ref() {
        #[derive(Debug, PartialEq)]
        struct Inner;
        struct Outer(Inner);

        impl AsRef<Inner> for Outer {
            fn as_ref(&self) -> &Inner {
                &self.0
            }
        }

        assert_eq!(&Inner, Secret::new(Outer(Inner)).expose_as_ref());
    }

    #[test]
    fn should_be_able_to_expose_as_clone() {
        #[derive(Clone, Debug, PartialEq)]
        struct Inner;
        #[derive(Clone)]
        struct Outer(Inner);

        impl AsClone<Inner> for &Outer {
            fn as_clone(self) -> Inner {
                self.0.clone()
            }
        }

        let secret = Secret::new(Outer(Inner));

        assert_eq!(Inner, secret.expose_as_clone());
    }

    #[test]
    fn should_be_able_to_expose_as_copy() {
        #[derive(Clone, Copy, Debug, PartialEq)]
        struct Inner;
        #[derive(Clone, Copy)]
        struct Outer(Inner);

        impl AsCopy<Inner> for &Outer {
            fn as_copy(self) -> Inner {
                self.0
            }
        }

        let secret = Secret::new(Outer(Inner));

        assert_eq!(Inner, secret.expose_as_copy());
    }
}
