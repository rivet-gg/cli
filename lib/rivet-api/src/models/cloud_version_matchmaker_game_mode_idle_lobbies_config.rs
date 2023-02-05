/*
 * Rivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CloudVersionMatchmakerGameModeIdleLobbiesConfig : Configuration for how many idle lobbies a game version should have.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CloudVersionMatchmakerGameModeIdleLobbiesConfig {
    #[serde(rename = "max", skip_serializing_if = "Option::is_none")]
    pub max: Option<i32>,
    #[serde(rename = "min", skip_serializing_if = "Option::is_none")]
    pub min: Option<i32>,
}

impl CloudVersionMatchmakerGameModeIdleLobbiesConfig {
    /// Configuration for how many idle lobbies a game version should have.
    pub fn new() -> CloudVersionMatchmakerGameModeIdleLobbiesConfig {
        CloudVersionMatchmakerGameModeIdleLobbiesConfig {
            max: None,
            min: None,
        }
    }
}

