//! Contains the `Secret<T>` struct, designed to wrap values that should never appear formatted with Display or Debug in logs or other output.

/// Wraps a value that should never appear formatted with Display or Debug in logs or other output.
///
/// `Secret<T>` does implement `Debug`, but only outputs the inner type name, ie. `"Secret<String>"`;
///
/// Forwards the following implementations from its inner type:
/// - `Clone`
/// - `Copy`
/// - `PartialeEq`
/// - `Eq`
/// - `PartialOrd`
/// - `Ord`
/// - `serde::Serialize` (if feature `"serde"` is active)
/// - `serde::Deserialize` (if feature `"serde"` is active)
/// - `schemars::JsonSchema` (if feature `"schemars"` is active)
///
#[cfg_attr(feature = "sqlx", derive(::sqlx::Type))]
#[cfg_attr(feature = "sqlx", sqlx(transparent))]
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

impl<T> From<T> for Secret<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl<T> std::fmt::Debug for Secret<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Secret<{}>", std::any::type_name::<T>()))
    }
}

// Serde impls:

#[cfg(feature = "serde")]
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

// Schemars impls:

#[cfg(feature = "schemars")]
impl<T: schemars::JsonSchema> schemars::JsonSchema for Secret<T> {
    fn schema_id() -> std::borrow::Cow<'static, str> {
        T::schema_id()
    }

    fn schema_name() -> std::borrow::Cow<'static, str> {
        T::schema_name()
    }

    fn inline_schema() -> bool {
        T::inline_schema()
    }

    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        T::json_schema(generator)
    }
}

// std impls:

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
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<T: Ord> Ord for Secret<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl<T: std::hash::Hash> std::hash::Hash for Secret<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state)
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
