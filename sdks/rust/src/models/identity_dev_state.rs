/*
 * Rivet API EE
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 *
 * Generated by: https://openapi-generator.tech
 */

/// IdentityDevState : The state of the given identity's developer status.

/// The state of the given identity's developer status.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum IdentityDevState {
	#[serde(rename = "inactive")]
	Inactive,
	#[serde(rename = "pending")]
	Pending,
	#[serde(rename = "accepted")]
	Accepted,
}

impl ToString for IdentityDevState {
	fn to_string(&self) -> String {
		match self {
			Self::Inactive => String::from("inactive"),
			Self::Pending => String::from("pending"),
			Self::Accepted => String::from("accepted"),
		}
	}
}

impl Default for IdentityDevState {
	fn default() -> IdentityDevState {
		Self::Inactive
	}
}
