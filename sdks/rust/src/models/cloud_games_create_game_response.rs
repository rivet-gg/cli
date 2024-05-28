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
pub struct CloudGamesCreateGameResponse {
    #[serde(rename = "game_id")]
    pub game_id: uuid::Uuid,
}

impl CloudGamesCreateGameResponse {
    pub fn new(game_id: uuid::Uuid) -> CloudGamesCreateGameResponse {
        CloudGamesCreateGameResponse {
            game_id,
        }
    }
}

