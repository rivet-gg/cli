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
pub struct IdentitySetGameActivityRequest {
    #[serde(rename = "game_activity")]
    pub game_activity: Box<crate::models::IdentityUpdateGameActivity>,
}

impl IdentitySetGameActivityRequest {
    pub fn new(game_activity: crate::models::IdentityUpdateGameActivity) -> IdentitySetGameActivityRequest {
        IdentitySetGameActivityRequest {
            game_activity: Box::new(game_activity),
        }
    }
}


