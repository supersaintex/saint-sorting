//! # Low Level API to convert between rust types and the Firebase REST API
//! Low level API to convert between generated rust types (see [`crate::dto`]) and
//! the data types of the Firebase REST API. Those are 1:1 translations of the grpc API
//! and deeply nested and wrapped.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use super::dto;
use super::errors::{FirebaseError, Result};

#[derive(Serialize, Deserialize)]
struct Wrapper {
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

use serde_json::{map::Map, Number};

/// Converts a flat serde json value into a firebase google-rpc-api inspired heavily nested and wrapped type
/// to be consumed by the Firebase REST API.
///
/// This is a low level API. You probably want to use [`crate::documents`] instead.
///
/// This method works recursively!
pub(crate) fn serde_value_to_firebase_value(v: &serde_json::Value) -> dto::Value {
    if v.is_f64() {
        return dto::Value {
            double_value: Some(v.as_f64().unwrap()),
            ..Default::default()
        };
    } else if let Some(integer_value) = v.as_i64() {
        return dto::Value {
            integer_value: Some(integer_value.to_string()),
            ..Default::default()
        };
    } else if let Some(map_value) = v.as_object() {
        let mut map: HashMap<String, dto::Value> = HashMap::new();
        for (map_key, map_v) in map_value {
            map.insert(map_key.to_owned(), serde_value_to_firebase_value(&map_v));
        }
        return dto::Value {
            map_value: Some(dto::MapValue { fields: Some(map) }),
            ..Default::default()
        };
    } else if let Some(string_value) = v.as_str() {
        return dto::Value {
            string_value: Some(string_value.to_owned()),
            ..Default::default()
        };
    } else if let Some(boolean_value) = v.as_bool() {
        return dto::Value {
            boolean_value: Some(boolean_value),
            ..Default::default()
        };
    } else if let Some(array_value) = v.as_array() {
        let mut vec: Vec<dto::Value> = Vec::new();
        for k in array_value {
            vec.push(serde_value_to_firebase_value(&k));
        }
        return dto::Value {
            array_value: Some(dto::ArrayValue { values: Some(vec) }),
            ..Default::default()
        };
    }
    Default::default()
}


/// Converts a custom data type into a firebase google-rpc-api inspired heavily nested and wrapped type
/// to be consumed by the Firebase REST API.
///
/// This is a low level API. You probably want to use [`crate::documents`] instead.
///
/// Internals:
///
/// This method uses recursion to decode the given firebase type.
pub fn pod_to_document<T>(pod: &T) -> Result<dto::Document>
where
    T: Serialize,
{
    let v = serde_json::to_value(pod)?;
    Ok(dto::Document {
        fields: serde_value_to_firebase_value(&v).map_value.unwrap().fields,
        ..Default::default()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    use super::Result;
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;

    #[derive(Serialize, Deserialize)]
    struct DemoPod {
        integer_test: u32,
        boolean_test: bool,
        string_test: String,
    }

    #[test]
    fn test_pod_to_document() -> Result<()> {
        let t = DemoPod {
            integer_test: 12,
            boolean_test: true,
            string_test: "abc".to_owned(),
        };
        let firebase_doc = pod_to_document(&t)?;
        let map = firebase_doc.fields;
        assert_eq!(
            map.unwrap()
                .get("integer_test")
                .expect("a value in the map for integer_test")
                .integer_value
                .as_ref()
                .expect("an integer value"),
            "12"
        );

        Ok(())
    }
}
