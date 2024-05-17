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
pub struct AdminClustersCreateRequest {
    #[serde(rename = "name_id")]
    pub name_id: String,
    #[serde(rename = "owner_team_id", skip_serializing_if = "Option::is_none")]
    pub owner_team_id: Option<uuid::Uuid>,
}

impl AdminClustersCreateRequest {
    pub fn new(name_id: String) -> AdminClustersCreateRequest {
        AdminClustersCreateRequest {
            name_id,
            owner_team_id: None,
        }
    }
}


