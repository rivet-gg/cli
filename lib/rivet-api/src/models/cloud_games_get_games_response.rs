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
pub struct CloudGamesGetGamesResponse {
	/// A list of game summaries.
	#[serde(rename = "games")]
	pub games: Vec<crate::models::CloudGamesGameSummary>,
	/// A list of group summaries.
	#[serde(rename = "groups")]
	pub groups: Vec<crate::models::GroupHandle>,
	#[serde(rename = "watch")]
	pub watch: Box<crate::models::WatchResponse>,
}

impl CloudGamesGetGamesResponse {
	pub fn new(
		games: Vec<crate::models::CloudGamesGameSummary>,
		groups: Vec<crate::models::GroupHandle>,
		watch: crate::models::WatchResponse,
	) -> CloudGamesGetGamesResponse {
		CloudGamesGetGamesResponse {
			games,
			groups,
			watch: Box::new(watch),
		}
	}
}
