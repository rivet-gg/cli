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
pub struct ServersDestroyServerResponse {
    #[serde(rename = "server_id")]
    pub server_id: uuid::Uuid,
}

impl ServersDestroyServerResponse {
    pub fn new(server_id: uuid::Uuid) -> ServersDestroyServerResponse {
        ServersDestroyServerResponse {
            server_id,
        }
    }
}


