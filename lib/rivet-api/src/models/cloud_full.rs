/*
 * Rivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CloudFull : A full version.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CloudFull {
    #[serde(rename = "config")]
    pub config: Box<crate::models::CloudConfig>,
    /// RFC3339 timestamp.
    #[serde(rename = "create_ts")]
    pub create_ts: String,
    /// Represent a resource's readable display name.
    #[serde(rename = "display_name")]
    pub display_name: String,
    /// A universally unique identifier.
    #[serde(rename = "version_id")]
    pub version_id: String,
}

impl CloudFull {
    /// A full version.
    pub fn new(config: crate::models::CloudConfig, create_ts: String, display_name: String, version_id: String) -> CloudFull {
        CloudFull {
            config: Box::new(config),
            create_ts,
            display_name,
            version_id,
        }
    }
}

