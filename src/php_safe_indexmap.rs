use indexmap::IndexMap;
use serde::{
    Deserialize,
    de::{self, Deserializer, MapAccess, Visitor},
};
use std::{fmt::Debug, hash::Hash, marker::PhantomData};

pub struct PhpSafeIndexMap<K, V>(PhantomData<(K, V)>);

impl<'de, K: Hash + Eq + Debug + Deserialize<'de>, V: Deserialize<'de>> PhpSafeIndexMap<K, V> {
    pub fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<IndexMap<K, V>, D::Error> {
        deserializer.deserialize_any(PhpSafeIndexMap::default())
    }
}

impl<K, V> Default for PhpSafeIndexMap<K, V> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<'de, K: Hash + Eq + Debug, V> Visitor<'de> for PhpSafeIndexMap<K, V>
where
    K: Deserialize<'de>,
    V: Deserialize<'de>,
{
    type Value = IndexMap<K, V>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("A IndexMap<K, V> or a sequence of key-value tuples like Vec<(K, V)> where each key must be unique")
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut map = IndexMap::with_capacity(access.size_hint().unwrap_or(0));

        while let Some((key, value)) = access.next_entry()? {
            map.insert(key, value);
        }

        Ok(map)
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let mut map = IndexMap::with_capacity(seq.size_hint().unwrap_or(0));

        while let Some((k, v)) = seq.next_element::<(K, V)>()? {
            if map.contains_key(&k) {
                return Err(de::Error::custom(format_args!("duplicate field `{k:?}`")));
            }

            map.insert(k, v);
        }

        Ok(map)
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Display;

    use super::*;
    use serde::Serialize;
    use serde_json::json;

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    #[serde(untagged)]
    enum StringOrUsize {
        String(String),
        Usize(usize),
    }

    impl From<usize> for StringOrUsize {
        fn from(value: usize) -> Self {
            Self::Usize(value)
        }
    }

    impl From<&'static str> for StringOrUsize {
        fn from(value: &'static str) -> Self {
            Self::String(value.to_string())
        }
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct MyPhpSafeMap(
        #[serde(deserialize_with = "PhpSafeIndexMap::deserialize")] IndexMap<String, StringOrUsize>,
    );

    impl MyPhpSafeMap {
        fn from(key_values: impl IntoIterator<Item = (impl Display, StringOrUsize)>) -> Self {
            let mut map = IndexMap::new();

            for (key, value) in key_values {
                map.insert(key.to_string(), value);
            }

            Self(map)
        }
    }

    #[test]
    fn should_work_with_an_actual_map() {
        let json = json! (
            {
                "foo": 123,
                "bar": "baz"
            }
        );

        let map: MyPhpSafeMap = serde_json::from_value(json).unwrap();

        assert_eq!(
            map,
            MyPhpSafeMap::from([("foo", 123.into()), ("bar", "baz".into())])
        )
    }

    #[test]
    fn should_work_with_an_associated_array() {
        let json = json!([["foo", 123], ["bar", "baz"]]);
        let map: MyPhpSafeMap = serde_json::from_value(json).unwrap();

        assert_eq!(
            map,
            MyPhpSafeMap::from([("foo", 123.into()), ("bar", "baz".into())])
        )
    }

    #[test]
    fn should_error_on_duplicate_fields() {
        let json = json!([["foo", 123], ["foo", "baz"]]);
        let res: Result<MyPhpSafeMap, _> = serde_json::from_value(json);

        assert!(res.is_err())
    }
}
