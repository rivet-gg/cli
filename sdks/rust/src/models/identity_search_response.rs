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
pub struct IdentitySearchResponse {
    /// The pagination anchor.
    #[serde(rename = "anchor", skip_serializing_if = "Option::is_none")]
    pub anchor: Option<String>,
    #[serde(rename = "identities")]
    pub identities: Vec<crate::models::IdentityHandle>,
}

impl IdentitySearchResponse {
    pub fn new(identities: Vec<crate::models::IdentityHandle>) -> IdentitySearchResponse {
        IdentitySearchResponse {
            anchor: None,
            identities,
        }
    }
}


