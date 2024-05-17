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
pub struct GroupCreateInviteRequest {
    /// How long until the group invite expires (in milliseconds).
    #[serde(rename = "ttl", skip_serializing_if = "Option::is_none")]
    pub ttl: Option<f64>,
    /// How many times the group invite can be used.
    #[serde(rename = "use_count", skip_serializing_if = "Option::is_none")]
    pub use_count: Option<f64>,
}

impl GroupCreateInviteRequest {
    pub fn new() -> GroupCreateInviteRequest {
        GroupCreateInviteRequest {
            ttl: None,
            use_count: None,
        }
    }
}


