/*
 * Rivet API EE
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 *
 * Generated by: https://openapi-generator.tech
 */

/// KvPutEntry : A new entry to insert into the key-value database.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct KvPutEntry {
	/// A string representing a key in the key-value database. Maximum length of 512 characters. _Recommended Key Path Format_ Key path components are split by a slash (e.g. `a/b/c` has the path components `[\"a\", \"b\", \"c\"]`). Slashes can be escaped by using a backslash (e.g. `a/b/c/d` has the path components `[\"a\", \"b/c\", \"d\"]`). This format is not enforced by Rivet, but the tools built around Rivet KV work better if this format is used.
	#[serde(rename = "key")]
	pub key: String,
	/// A JSON object stored in the KV database. A `null` value indicates the entry is deleted. Maximum length of 262,144 bytes when encoded.
	#[serde(rename = "value", deserialize_with = "Option::deserialize")]
	pub value: Option<serde_json::Value>,
}

impl KvPutEntry {
	/// A new entry to insert into the key-value database.
	pub fn new(key: String, value: Option<serde_json::Value>) -> KvPutEntry {
		KvPutEntry { key, value }
	}
}
