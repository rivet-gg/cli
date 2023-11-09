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
pub struct CloudGamesDeleteMatchmakerLobbyResponse {
	/// Whether or not the lobby was successfully stopped.
	#[serde(rename = "did_remove", skip_serializing_if = "Option::is_none")]
	pub did_remove: Option<bool>,
}

impl CloudGamesDeleteMatchmakerLobbyResponse {
	pub fn new() -> CloudGamesDeleteMatchmakerLobbyResponse {
		CloudGamesDeleteMatchmakerLobbyResponse { did_remove: None }
	}
}
