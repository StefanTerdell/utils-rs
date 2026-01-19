//! Contains the `Secret<T>` struct, designed to wrap values that should never appear formatted with Display or Debug in logs or other output.

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
pub struct Secret<T>(T);

impl<T> Secret<T> {
    pub fn new(value: T) -> Self {
        Self(value)
    }

    pub fn expose(self) -> T {
        self.0
    }

    pub fn expose_ref(&self) -> &T {
        &self.0
    }
}

impl<T: Clone> Secret<T> {
    pub fn expose_clone(&self) -> T {
        self.0.clone()
    }
}

impl<T: Copy> Secret<T> {
    pub fn expose_copy(&self) -> T {
        self.0
    }
}

impl<T> From<T> for Secret<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl<T: Clone> Clone for Secret<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T: Copy> Copy for Secret<T> {}

impl<T: PartialEq> PartialEq for Secret<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl<T: Eq> Eq for Secret<T> {}

impl<T: PartialOrd> PartialOrd for Secret<T> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<T: Ord> Ord for Secret<T> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl<T> core::fmt::Debug for Secret<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!("Secret<{}>", core::any::type_name::<T>()))
    }
}

impl<T: core::hash::Hash> core::hash::Hash for Secret<T> {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state)
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
            self.0.serialize(serializer)
        }
    }

    #[cfg(feature = "serde")]
    impl<'de, T: serde::Deserialize<'de>> serde::Deserialize<'de> for Secret<T> {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            T::deserialize(deserializer).map(Secret)
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
            T::json_schema(generator)
        }
    }
}

#[cfg(feature = "sqlx")]
mod sqlx_impls {
    impl<DB: sqlx::Database, T: sqlx::Type<DB>> sqlx::Type<DB> for Secret<T> {
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
    use super::Secret;

    #[test]
    fn debug_should_output_type_only() {
        let secret = Secret::new("Hello, world!");

        assert_eq!(format!("{secret:#?}"), "Secret<&str>");
    }
}
