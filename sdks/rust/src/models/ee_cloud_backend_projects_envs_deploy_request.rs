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
pub struct EeCloudBackendProjectsEnvsDeployRequest {
    #[serde(rename = "upload_id")]
    pub upload_id: uuid::Uuid,
}

impl EeCloudBackendProjectsEnvsDeployRequest {
    pub fn new(upload_id: uuid::Uuid) -> EeCloudBackendProjectsEnvsDeployRequest {
        EeCloudBackendProjectsEnvsDeployRequest {
            upload_id,
        }
    }
}


