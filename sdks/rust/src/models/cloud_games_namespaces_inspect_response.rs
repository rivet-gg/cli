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
pub struct CloudGamesNamespacesInspectResponse {
    #[serde(rename = "agent")]
    pub agent: Box<crate::models::CloudAuthAgent>,
}

impl CloudGamesNamespacesInspectResponse {
    pub fn new(agent: crate::models::CloudAuthAgent) -> CloudGamesNamespacesInspectResponse {
        CloudGamesNamespacesInspectResponse {
            agent: Box::new(agent),
        }
    }
}


