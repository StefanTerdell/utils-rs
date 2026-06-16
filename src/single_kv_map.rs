#[cfg(feature = "schemars")]
use schemars::{JsonSchema, json_schema};
use serde::de::{Error, MapAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize};
use std::hash::Hash;
use std::marker::PhantomData;

#[derive(Debug, Clone, Copy, Eq, PartialOrd, Ord, Hash)]
pub struct SingleKVMap<Key, Value> {
    pub key: Key,
    pub value: Value,
}

impl<LeftKey: PartialEq<RightKey>, LeftValue: PartialEq<RightValue>, RightKey, RightValue>
    PartialEq<SingleKVMap<RightKey, RightValue>> for SingleKVMap<LeftKey, LeftValue>
{
    fn eq(&self, other: &SingleKVMap<RightKey, RightValue>) -> bool {
        self.key == other.key && self.value == other.value
    }
}

impl<Key: Serialize, Value: Serialize> Serialize for SingleKVMap<Key, Value> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(Some(1))?;
        map.serialize_entry(&self.key, &self.value)?;
        map.end()
    }
}

struct SingleKVMapVisitor<Key, Value>(PhantomData<(Key, Value)>);

impl<Key, Value> Default for SingleKVMapVisitor<Key, Value> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<'de, Key: Deserialize<'de>, Value: Deserialize<'de>> Visitor<'de>
    for SingleKVMapVisitor<Key, Value>
{
    type Value = SingleKVMap<Key, Value>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a map with a single key-value")
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A::Error: Error,
    {
        let (key, value) = map
            .next_entry()?
            .ok_or(A::Error::missing_field("missing first (and only) entry"))?;

        if let Some(remaining) = map.size_hint()
            && remaining != 0
        {
            Err(A::Error::unknown_field(
                &format!("Found {remaining} remaining entries, expecting 0"),
                &[],
            ))
        } else {
            Ok(SingleKVMap::new(key, value))
        }
    }
}

impl<'de, Key: Deserialize<'de>, Value: Deserialize<'de>> Deserialize<'de>
    for SingleKVMap<Key, Value>
{
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_map(SingleKVMapVisitor::default())
    }
}

impl<Key, Value> SingleKVMap<Key, Value> {
    pub fn new(key: Key, value: Value) -> Self {
        Self { key, value }
    }

    pub fn from_entry(entry: (Key, Value)) -> Self {
        let (key, value) = entry;
        Self { key, value }
    }

    pub fn as_key(&self) -> &Key {
        &self.key
    }

    pub fn as_key_mut(&mut self) -> &mut Key {
        &mut self.key
    }

    pub fn into_key(self) -> Key {
        self.key
    }

    pub fn set_key(&mut self, new_key: Key) -> Key {
        std::mem::replace(&mut self.key, new_key)
    }

    pub fn as_value(&self) -> &Value {
        &self.value
    }

    pub fn as_value_mut(&mut self) -> &mut Value {
        &mut self.value
    }

    pub fn into_value(self) -> Value {
        self.value
    }

    pub fn set_value(&mut self, new_value: Value) -> Value {
        std::mem::replace(&mut self.value, new_value)
    }

    pub fn as_entry(&self) -> (&Key, &Value) {
        (&self.key, &self.value)
    }

    pub fn as_entry_mut(&mut self) -> (&mut Key, &mut Value) {
        (&mut self.key, &mut self.value)
    }

    pub fn into_entry(self) -> (Key, Value) {
        (self.key, self.value)
    }

    pub fn set_entry(&mut self, new_entry: (Key, Value)) -> (Key, Value) {
        let (new_key, new_value) = new_entry;
        (
            std::mem::replace(&mut self.key, new_key),
            std::mem::replace(&mut self.value, new_value),
        )
    }
}

impl<Key, Value> From<(Key, Value)> for SingleKVMap<Key, Value> {
    fn from(entry: (Key, Value)) -> Self {
        Self::from_entry(entry)
    }
}

impl<Key, Value> AsRef<SingleKVMap<Key, Value>> for SingleKVMap<Key, Value> {
    fn as_ref(&self) -> &SingleKVMap<Key, Value> {
        self
    }
}

#[cfg(feature = "schemars")]
impl<Key: JsonSchema, Value: JsonSchema> JsonSchema for SingleKVMap<Key, Value> {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        format!("{}{}SingleKVMap", Key::schema_name(), Value::schema_name()).into()
    }

    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        let mut key_schema = generator.subschema_for::<Key>();
        let value_schema = generator.subschema_for::<Value>();

        key_schema.as_object_mut().map(|x| x.remove("type"));

        json_schema!({
            "type": "object",
            "propertyNames": key_schema,
            "additionalProperties": value_schema,
            "minProperties": 1,
            "maxProperties": 1,
        })
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    #[cfg(feature = "schemars")]
    use schemars::schema_for;
    use serde_json::json;

    use super::*;

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    #[cfg_attr(feature = "schemars", derive(JsonSchema))]
    enum MyEnum {
        A,
        B,
    }

    #[cfg(feature = "schemars")]
    #[test]
    fn derive_json_schema() {
        assert_eq!(
            schema_for!(SingleKVMap<i32, String>),
            json_schema!({
                "$schema": "https://json-schema.org/draft/2020-12/schema",
                "title": "int32stringSingleKVMap",
                "type": "object",
                "additionalProperties": {
                    "type": "string"
                },
                "propertyNames": {
                    "format": "int32"
                },
                "minProperties": 1,
                "maxProperties": 1,
            })
        )
    }

    #[cfg(feature = "schemars")]
    #[test]
    fn derive_json_schema_enum() {
        assert_eq!(
            schema_for!(SingleKVMap<MyEnum, MyEnum>),
            json_schema!({
                "$schema": "https://json-schema.org/draft/2020-12/schema",
                "title": "MyEnumMyEnumSingleKVMap",
                "type": "object",
                "additionalProperties": {
                    "$ref": "#/$defs/MyEnum"
                },
                "propertyNames": {
                    "$ref": "#/$defs/MyEnum"
                },
                "minProperties": 1,
                "maxProperties": 1,
                "$defs": {
                    "MyEnum": {
                        "type": "string",
                        "enum": ["A", "B"]
                    }
                }
            })
        )
    }

    #[test]
    fn derive_serialize() {
        assert_eq!(
            serde_json::to_value(SingleKVMap::new(10, "hello")).unwrap(),
            json! ({
                "10": "hello"
            })
        )
    }

    #[test]
    fn derive_serialize_enum() {
        assert_eq!(
            serde_json::to_value(SingleKVMap::new(MyEnum::A, MyEnum::B)).unwrap(),
            json! ({
                "A": "B"
            })
        )
    }

    #[test]
    fn derive_deserialize() {
        assert_eq!(
            serde_json::from_value::<SingleKVMap<i32, String>>(json! ({
                "10": "hello"
            }))
            .unwrap(),
            SingleKVMap::new(10, "hello"),
        )
    }

    #[test]
    fn derive_deserialize_enum() {
        assert_eq!(
            serde_json::from_value::<SingleKVMap<MyEnum, MyEnum>>(json! ({
                "A": "B"
            }))
            .unwrap(),
            SingleKVMap::new(MyEnum::A, MyEnum::B),
        )
    }
}
