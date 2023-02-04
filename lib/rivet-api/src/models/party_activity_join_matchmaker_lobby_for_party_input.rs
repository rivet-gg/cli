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
pub struct PartyActivityJoinMatchmakerLobbyForPartyInput {
    /// A universally unique identifier.
    #[serde(rename = "lobby_id")]
    pub lobby_id: String,
    #[serde(rename = "captcha", skip_serializing_if = "Option::is_none")]
    pub captcha: Option<Box<crate::models::CommonsCaptchaConfig>>,
}

impl PartyActivityJoinMatchmakerLobbyForPartyInput {
    pub fn new(lobby_id: String) -> PartyActivityJoinMatchmakerLobbyForPartyInput {
        PartyActivityJoinMatchmakerLobbyForPartyInput {
            lobby_id,
            captcha: None,
        }
    }
}

