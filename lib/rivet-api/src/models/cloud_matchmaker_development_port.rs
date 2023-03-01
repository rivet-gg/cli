/*
 * Rivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CloudMatchmakerDevelopmentPort : A port configuration used to create development tokens.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CloudMatchmakerDevelopmentPort {
    #[serde(rename = "port")]
    pub port: i32,
    #[serde(rename = "protocol")]
    pub protocol: crate::models::CloudVersionMatchmakerPortProtocol,
}

impl CloudMatchmakerDevelopmentPort {
    /// A port configuration used to create development tokens.
    pub fn new(port: i32, protocol: crate::models::CloudVersionMatchmakerPortProtocol) -> CloudMatchmakerDevelopmentPort {
        CloudMatchmakerDevelopmentPort {
            port,
            protocol,
        }
    }
}


