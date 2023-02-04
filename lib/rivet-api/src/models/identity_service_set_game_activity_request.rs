/*
 * Rivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct IdentityServiceSetGameActivityRequest {
    #[serde(rename = "game_activity")]
    pub game_activity: Box<crate::models::IdentityUpdateIdentityGameActivity>,
}

impl IdentityServiceSetGameActivityRequest {
    pub fn new(game_activity: crate::models::IdentityUpdateIdentityGameActivity) -> IdentityServiceSetGameActivityRequest {
        IdentityServiceSetGameActivityRequest {
            game_activity: Box::new(game_activity),
        }
    }
}

