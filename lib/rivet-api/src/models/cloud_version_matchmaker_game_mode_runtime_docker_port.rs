/*
 * Rivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CloudVersionMatchmakerGameModeRuntimeDockerPort : A docker port.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CloudVersionMatchmakerGameModeRuntimeDockerPort {
    /// Client-side configuration
    #[serde(rename = "dev_port", skip_serializing_if = "Option::is_none")]
    pub dev_port: Option<i32>,
    #[serde(rename = "dev_protocol", skip_serializing_if = "Option::is_none")]
    pub dev_protocol: Option<crate::models::CloudVersionMatchmakerPortProtocol>,
    /// The port number to connect to.
    #[serde(rename = "port", skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "port_range", skip_serializing_if = "Option::is_none")]
    pub port_range: Option<Box<crate::models::CloudVersionMatchmakerPortRange>>,
    #[serde(rename = "protocol", skip_serializing_if = "Option::is_none")]
    pub protocol: Option<crate::models::CloudVersionMatchmakerPortProtocol>,
    #[serde(rename = "proxy", skip_serializing_if = "Option::is_none")]
    pub proxy: Option<crate::models::CloudVersionMatchmakerProxyKind>,
}

impl CloudVersionMatchmakerGameModeRuntimeDockerPort {
    /// A docker port.
    pub fn new() -> CloudVersionMatchmakerGameModeRuntimeDockerPort {
        CloudVersionMatchmakerGameModeRuntimeDockerPort {
            dev_port: None,
            dev_protocol: None,
            port: None,
            port_range: None,
            protocol: None,
            proxy: None,
        }
    }
}


