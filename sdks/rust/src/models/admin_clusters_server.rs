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
pub struct AdminClustersServer {
    #[serde(rename = "public_ip", skip_serializing_if = "Option::is_none")]
    pub public_ip: Option<String>,
    #[serde(rename = "server_id")]
    pub server_id: uuid::Uuid,
}

impl AdminClustersServer {
    pub fn new(server_id: uuid::Uuid) -> AdminClustersServer {
        AdminClustersServer {
            public_ip: None,
            server_id,
        }
    }
}


