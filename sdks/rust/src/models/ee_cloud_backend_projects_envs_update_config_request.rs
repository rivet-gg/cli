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
pub struct EeCloudBackendProjectsEnvsUpdateConfigRequest {
	#[serde(rename = "config")]
	pub config: Box<crate::models::EeBackendNeonProjectConfig>,
}

impl EeCloudBackendProjectsEnvsUpdateConfigRequest {
	pub fn new(
		config: crate::models::EeBackendNeonProjectConfig,
	) -> EeCloudBackendProjectsEnvsUpdateConfigRequest {
		EeCloudBackendProjectsEnvsUpdateConfigRequest {
			config: Box::new(config),
		}
	}
}