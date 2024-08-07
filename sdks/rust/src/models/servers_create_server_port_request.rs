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
pub struct ServersCreateServerPortRequest {
    #[serde(rename = "protocol")]
    pub protocol: crate::models::ServersPortProtocol,
    #[serde(rename = "routing", skip_serializing_if = "Option::is_none")]
    pub routing: Option<Box<crate::models::ServersPortRouting>>,
    #[serde(rename = "server_port", skip_serializing_if = "Option::is_none")]
    pub server_port: Option<i32>,
}

impl ServersCreateServerPortRequest {
    pub fn new(protocol: crate::models::ServersPortProtocol) -> ServersCreateServerPortRequest {
        ServersCreateServerPortRequest {
            protocol,
            routing: None,
            server_port: None,
        }
    }
}


