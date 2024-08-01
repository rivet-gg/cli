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
pub struct MatchmakerJoinPort {
    /// The host for the given port. Will be null if using a port range.
    #[serde(rename = "host", skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    #[serde(rename = "hostname")]
    pub hostname: String,
    /// Whether or not this lobby port uses TLS. You cannot mix a non-TLS and TLS ports.
    #[serde(rename = "is_tls")]
    pub is_tls: bool,
    /// The port number for this lobby. Will be null if using a port range.
    #[serde(rename = "port", skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "port_range", skip_serializing_if = "Option::is_none")]
    pub port_range: Option<Box<crate::models::MatchmakerJoinPortRange>>,
}

impl MatchmakerJoinPort {
    pub fn new(hostname: String, is_tls: bool) -> MatchmakerJoinPort {
        MatchmakerJoinPort {
            host: None,
            hostname,
            is_tls,
            port: None,
            port_range: None,
        }
    }
}


