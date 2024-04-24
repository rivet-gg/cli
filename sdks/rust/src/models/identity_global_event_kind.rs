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
pub struct IdentityGlobalEventKind {
	#[serde(rename = "identity_update", skip_serializing_if = "Option::is_none")]
	pub identity_update: Option<Box<crate::models::IdentityGlobalEventIdentityUpdate>>,
	#[serde(
		rename = "matchmaker_lobby_join",
		skip_serializing_if = "Option::is_none"
	)]
	pub matchmaker_lobby_join: Option<Box<crate::models::IdentityGlobalEventMatchmakerLobbyJoin>>,
}

impl IdentityGlobalEventKind {
	pub fn new() -> IdentityGlobalEventKind {
		IdentityGlobalEventKind {
			identity_update: None,
			matchmaker_lobby_join: None,
		}
	}
}
