/*
 * Rivet API EE
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CloudVersionMatchmakerGameModeActions : Configuration for the connection types allowed for a game mode.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CloudVersionMatchmakerGameModeActions {
    #[serde(rename = "create", skip_serializing_if = "Option::is_none")]
    pub create: Option<Box<crate::models::CloudVersionMatchmakerGameModeCreateConfig>>,
    #[serde(rename = "find", skip_serializing_if = "Option::is_none")]
    pub find: Option<Box<crate::models::CloudVersionMatchmakerGameModeFindConfig>>,
    #[serde(rename = "join", skip_serializing_if = "Option::is_none")]
    pub join: Option<Box<crate::models::CloudVersionMatchmakerGameModeJoinConfig>>,
}

impl CloudVersionMatchmakerGameModeActions {
    /// Configuration for the connection types allowed for a game mode.
    pub fn new() -> CloudVersionMatchmakerGameModeActions {
        CloudVersionMatchmakerGameModeActions {
            create: None,
            find: None,
            join: None,
        }
    }
}


