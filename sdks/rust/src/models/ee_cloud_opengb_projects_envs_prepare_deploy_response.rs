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
pub struct EeCloudOpengbProjectsEnvsPrepareDeployResponse {
    #[serde(rename = "presigned_requests")]
    pub presigned_requests: Vec<crate::models::UploadPresignedRequest>,
    #[serde(rename = "upload_id")]
    pub upload_id: uuid::Uuid,
}

impl EeCloudOpengbProjectsEnvsPrepareDeployResponse {
    pub fn new(presigned_requests: Vec<crate::models::UploadPresignedRequest>, upload_id: uuid::Uuid) -> EeCloudOpengbProjectsEnvsPrepareDeployResponse {
        EeCloudOpengbProjectsEnvsPrepareDeployResponse {
            presigned_requests,
            upload_id,
        }
    }
}


