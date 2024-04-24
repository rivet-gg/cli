/*
 * Rivet API EE
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 *
 * Generated by: https://openapi-generator.tech
 */

/// IdentityProfile : An identity profile.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct IdentityProfile {
	#[serde(rename = "account_number")]
	pub account_number: i32,
	/// The URL of this identity's avatar image.
	#[serde(rename = "avatar_url")]
	pub avatar_url: String,
	/// Whether or not this identity is awaiting account deletion. Only visible to when the requestee is this identity.
	#[serde(rename = "awaiting_deletion", skip_serializing_if = "Option::is_none")]
	pub awaiting_deletion: Option<bool>,
	/// Follows regex ^(?:[^\\n\\r]+\\n?|\\n){1,5}$
	#[serde(rename = "bio")]
	pub bio: String,
	#[serde(rename = "dev_state", skip_serializing_if = "Option::is_none")]
	pub dev_state: Option<crate::models::IdentityDevState>,
	/// Represent a resource's readable display name.
	#[serde(rename = "display_name")]
	pub display_name: String,
	#[serde(rename = "external")]
	pub external: Box<crate::models::IdentityExternalLinks>,
	#[serde(rename = "follower_count")]
	pub follower_count: i64,
	/// Whether or not the requestee's identity is following this identity.
	#[serde(rename = "following")]
	pub following: bool,
	#[serde(rename = "following_count")]
	pub following_count: i64,
	#[serde(rename = "games")]
	pub games: Vec<crate::models::GameStatSummary>,
	#[serde(rename = "groups")]
	pub groups: Vec<crate::models::IdentityGroup>,
	#[serde(rename = "identity_id")]
	pub identity_id: uuid::Uuid,
	/// Whether or not this identity is an admin.
	#[serde(rename = "is_admin")]
	pub is_admin: bool,
	/// Whether or not this identity is both following and is followed by the requestee's identity.
	#[serde(rename = "is_following_me")]
	pub is_following_me: bool,
	/// Whether or not this game user has been linked through the Rivet dashboard.
	#[serde(rename = "is_game_linked", skip_serializing_if = "Option::is_none")]
	pub is_game_linked: Option<bool>,
	#[serde(rename = "is_mutual_following")]
	pub is_mutual_following: bool,
	/// Whether or not this identity is registered with a linked account.
	#[serde(rename = "is_registered")]
	pub is_registered: bool,
	/// RFC3339 timestamp
	#[serde(rename = "join_ts")]
	pub join_ts: String,
	#[serde(rename = "linked_accounts")]
	pub linked_accounts: Vec<crate::models::IdentityLinkedAccount>,
	#[serde(rename = "presence", skip_serializing_if = "Option::is_none")]
	pub presence: Option<Box<crate::models::IdentityPresence>>,
}

impl IdentityProfile {
	/// An identity profile.
	pub fn new(
		account_number: i32,
		avatar_url: String,
		bio: String,
		display_name: String,
		external: crate::models::IdentityExternalLinks,
		follower_count: i64,
		following: bool,
		following_count: i64,
		games: Vec<crate::models::GameStatSummary>,
		groups: Vec<crate::models::IdentityGroup>,
		identity_id: uuid::Uuid,
		is_admin: bool,
		is_following_me: bool,
		is_mutual_following: bool,
		is_registered: bool,
		join_ts: String,
		linked_accounts: Vec<crate::models::IdentityLinkedAccount>,
	) -> IdentityProfile {
		IdentityProfile {
			account_number,
			avatar_url,
			awaiting_deletion: None,
			bio,
			dev_state: None,
			display_name,
			external: Box::new(external),
			follower_count,
			following,
			following_count,
			games,
			groups,
			identity_id,
			is_admin,
			is_following_me,
			is_game_linked: None,
			is_mutual_following,
			is_registered,
			join_ts,
			linked_accounts,
			presence: None,
		}
	}
}
