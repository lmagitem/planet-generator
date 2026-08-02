//! Serde helpers for the galaxy's ordered cache maps (`BTreeMap`s keyed by
//! coordinates/levels). `serde_json` cannot serialize maps with non-string
//! keys ("key must be a string"), so the maps are written as a sequence of
//! `(key, value)` pairs instead, which also preserves the map's sorted order.
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::BTreeMap;

/// Serializes a [`BTreeMap`] as a sequence of `(key, value)` pairs, in key order.
pub fn serialize<K, V, S>(map: &BTreeMap<K, V>, serializer: S) -> Result<S::Ok, S::Error>
where
    K: Serialize,
    V: Serialize,
    S: Serializer,
{
    map.iter().collect::<Vec<_>>().serialize(serializer)
}

/// Deserializes a [`BTreeMap`] from the pair sequence written by [`serialize`].
pub fn deserialize<'de, K, V, D>(deserializer: D) -> Result<BTreeMap<K, V>, D::Error>
where
    K: Deserialize<'de> + Ord,
    V: Deserialize<'de>,
    D: Deserializer<'de>,
{
    Ok(Vec::<(K, V)>::deserialize(deserializer)?
        .into_iter()
        .collect())
}
