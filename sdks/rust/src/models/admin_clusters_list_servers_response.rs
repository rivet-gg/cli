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
pub struct AdminClustersListServersResponse {
    #[serde(rename = "servers")]
    pub servers: Vec<crate::models::AdminClustersServer>,
}

impl AdminClustersListServersResponse {
    pub fn new(servers: Vec<crate::models::AdminClustersServer>) -> AdminClustersListServersResponse {
        AdminClustersListServersResponse {
            servers,
        }
    }
}


