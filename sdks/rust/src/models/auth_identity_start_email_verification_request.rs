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
pub struct AuthIdentityStartEmailVerificationRequest {
	#[serde(rename = "captcha", skip_serializing_if = "Option::is_none")]
	pub captcha: Option<Box<crate::models::CaptchaConfig>>,
	#[serde(rename = "email")]
	pub email: String,
	#[serde(rename = "game_id", skip_serializing_if = "Option::is_none")]
	pub game_id: Option<uuid::Uuid>,
}

impl AuthIdentityStartEmailVerificationRequest {
	pub fn new(email: String) -> AuthIdentityStartEmailVerificationRequest {
		AuthIdentityStartEmailVerificationRequest {
			captcha: None,
			email,
			game_id: None,
		}
	}
}
