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
pub struct EeCloudOpengbProjectsEnvsPrepareDeployRequest {
    /// A list of files preparing to upload.
    #[serde(rename = "files")]
    pub files: Vec<crate::models::UploadPrepareFile>,
}

impl EeCloudOpengbProjectsEnvsPrepareDeployRequest {
    pub fn new(files: Vec<crate::models::UploadPrepareFile>) -> EeCloudOpengbProjectsEnvsPrepareDeployRequest {
        EeCloudOpengbProjectsEnvsPrepareDeployRequest {
            files,
        }
    }
}


