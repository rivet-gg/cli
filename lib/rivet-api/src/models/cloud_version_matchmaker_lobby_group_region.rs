/*
 * Rivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CloudVersionMatchmakerLobbyGroupRegion : **Deprecated: use GameMode instead** A game mode region.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CloudVersionMatchmakerLobbyGroupRegion {
    #[serde(rename = "idle_lobbies", skip_serializing_if = "Option::is_none")]
    pub idle_lobbies: Option<Box<crate::models::CloudVersionMatchmakerLobbyGroupIdleLobbiesConfig>>,
    /// A universally unique identifier.
    #[serde(rename = "region_id")]
    pub region_id: String,
    /// A human readable short identifier used to references resources. Different than a `rivet.common#Uuid` because this is intended to be human readable. Different than `rivet.common#DisplayName` because this should not include special characters and be short.
    #[serde(rename = "tier_name_id")]
    pub tier_name_id: String,
}

impl CloudVersionMatchmakerLobbyGroupRegion {
    /// **Deprecated: use GameMode instead** A game mode region.
    pub fn new(region_id: String, tier_name_id: String) -> CloudVersionMatchmakerLobbyGroupRegion {
        CloudVersionMatchmakerLobbyGroupRegion {
            idle_lobbies: None,
            region_id,
            tier_name_id,
        }
    }
}


