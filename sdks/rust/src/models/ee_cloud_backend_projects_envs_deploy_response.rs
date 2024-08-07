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
pub struct EeCloudBackendProjectsEnvsDeployResponse {
    /// Url to access the deployed OpenGB environment from.
    #[serde(rename = "url")]
    pub url: String,
}

impl EeCloudBackendProjectsEnvsDeployResponse {
    pub fn new(url: String) -> EeCloudBackendProjectsEnvsDeployResponse {
        EeCloudBackendProjectsEnvsDeployResponse {
            url,
        }
    }
}


