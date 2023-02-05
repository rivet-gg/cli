/*
 * Rivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CloudVersionLobbyGroup : A game mode.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CloudVersionLobbyGroup {
    /// Unsigned 32 bit integer.
    #[serde(rename = "max_players_direct", skip_serializing_if = "Option::is_none")]
    pub max_players_direct: Option<f64>,
    /// Unsigned 32 bit integer.
    #[serde(rename = "max_players_normal", skip_serializing_if = "Option::is_none")]
    pub max_players_normal: Option<f64>,
    /// Unsigned 32 bit integer.
    #[serde(rename = "max_players_party", skip_serializing_if = "Option::is_none")]
    pub max_players_party: Option<f64>,
    /// A human readable short identifier used to references resources. Different than a `rivet.common#Uuid` because this is intended to be human readable. Different than `rivet.common#DisplayName` because this should not include special characters and be short.
    #[serde(rename = "name_id")]
    pub name_id: String,
    /// A list of game mode regions.
    #[serde(rename = "regions")]
    pub regions: Vec<crate::models::CloudVersionLobbyGroupRegion>,
    #[serde(rename = "runtime")]
    pub runtime: Box<crate::models::CloudVersionLobbyGroupRuntime>,
}

impl CloudVersionLobbyGroup {
    /// A game mode.
    pub fn new(name_id: String, regions: Vec<crate::models::CloudVersionLobbyGroupRegion>, runtime: crate::models::CloudVersionLobbyGroupRuntime) -> CloudVersionLobbyGroup {
        CloudVersionLobbyGroup {
            max_players_direct: None,
            max_players_normal: None,
            max_players_party: None,
            name_id,
            regions,
            runtime: Box::new(runtime),
        }
    }
}

