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
pub struct CloudGamesUpdateGameNamespaceMatchmakerConfigInput {
    /// Unsigned 32 bit integer.
    #[serde(rename = "lobby_count_max", skip_serializing_if = "Option::is_none")]
    pub lobby_count_max: Option<f64>,
    /// Unsigned 32 bit integer.
    #[serde(rename = "max_players", skip_serializing_if = "Option::is_none")]
    pub max_players: Option<f64>,
}

impl CloudGamesUpdateGameNamespaceMatchmakerConfigInput {
    pub fn new() -> CloudGamesUpdateGameNamespaceMatchmakerConfigInput {
        CloudGamesUpdateGameNamespaceMatchmakerConfigInput {
            lobby_count_max: None,
            max_players: None,
        }
    }
}

