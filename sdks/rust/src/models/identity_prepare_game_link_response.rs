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
pub struct IdentityPrepareGameLinkResponse {
    /// RFC3339 timestamp
    #[serde(rename = "expire_ts")]
    pub expire_ts: String,
    /// Pass this to `GetGameLink` to get the linking status. Valid for 15 minutes.
    #[serde(rename = "identity_link_token")]
    pub identity_link_token: String,
    #[serde(rename = "identity_link_url")]
    pub identity_link_url: String,
}

impl IdentityPrepareGameLinkResponse {
    pub fn new(expire_ts: String, identity_link_token: String, identity_link_url: String) -> IdentityPrepareGameLinkResponse {
        IdentityPrepareGameLinkResponse {
            expire_ts,
            identity_link_token,
            identity_link_url,
        }
    }
}


