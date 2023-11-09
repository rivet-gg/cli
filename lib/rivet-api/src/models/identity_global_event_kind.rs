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
pub struct IdentityGlobalEventKind {
	#[serde(rename = "chat_message", skip_serializing_if = "Option::is_none")]
	pub chat_message: Option<Box<crate::models::IdentityGlobalEventChatMessage>>,
	#[serde(rename = "chat_read", skip_serializing_if = "Option::is_none")]
	pub chat_read: Option<Box<crate::models::IdentityGlobalEventChatRead>>,
	#[serde(rename = "chat_thread_remove", skip_serializing_if = "Option::is_none")]
	pub chat_thread_remove: Option<Box<crate::models::IdentityGlobalEventChatThreadRemove>>,
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
			chat_message: None,
			chat_read: None,
			chat_thread_remove: None,
			identity_update: None,
			matchmaker_lobby_join: None,
		}
	}
}
