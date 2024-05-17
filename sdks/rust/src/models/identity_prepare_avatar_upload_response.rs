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
pub struct IdentityPrepareAvatarUploadResponse {
    #[serde(rename = "presigned_request")]
    pub presigned_request: Box<crate::models::UploadPresignedRequest>,
    #[serde(rename = "upload_id")]
    pub upload_id: uuid::Uuid,
}

impl IdentityPrepareAvatarUploadResponse {
    pub fn new(presigned_request: crate::models::UploadPresignedRequest, upload_id: uuid::Uuid) -> IdentityPrepareAvatarUploadResponse {
        IdentityPrepareAvatarUploadResponse {
            presigned_request: Box::new(presigned_request),
            upload_id,
        }
    }
}


