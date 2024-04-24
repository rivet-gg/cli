/*
 * Rivet API EE
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 *
 * Generated by: https://openapi-generator.tech
 */

/// CloudBootstrapDomains : Domains that host parts of Rivet

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CloudBootstrapDomains {
	#[serde(rename = "cdn")]
	pub cdn: String,
	#[serde(rename = "job")]
	pub job: String,
	#[serde(rename = "main")]
	pub main: String,
}

impl CloudBootstrapDomains {
	/// Domains that host parts of Rivet
	pub fn new(cdn: String, job: String, main: String) -> CloudBootstrapDomains {
		CloudBootstrapDomains { cdn, job, main }
	}
}
