/*
 * Rivet API EE
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct KvListResponse {
	#[serde(rename = "entries")]
	pub entries: Vec<crate::models::KvEntry>,
}

impl KvListResponse {
	pub fn new(entries: Vec<crate::models::KvEntry>) -> KvListResponse {
		KvListResponse { entries }
	}
}
