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
pub struct CloudVersionCdnRoute {
	#[serde(rename = "glob")]
	pub glob: String,
	/// Multiple CDN version middleware.
	#[serde(rename = "middlewares")]
	pub middlewares: Vec<crate::models::CloudVersionCdnMiddleware>,
	/// Unsigned 32 bit integer.
	#[serde(rename = "priority")]
	pub priority: i32,
}

impl CloudVersionCdnRoute {
	pub fn new(
		glob: String,
		middlewares: Vec<crate::models::CloudVersionCdnMiddleware>,
		priority: i32,
	) -> CloudVersionCdnRoute {
		CloudVersionCdnRoute {
			glob,
			middlewares,
			priority,
		}
	}
}
