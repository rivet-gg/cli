/*
 * Rivet API EE
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CaptchaConfig : Methods to verify a captcha



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CaptchaConfig {
    #[serde(rename = "hcaptcha", skip_serializing_if = "Option::is_none")]
    pub hcaptcha: Option<Box<crate::models::CaptchaConfigHcaptcha>>,
    #[serde(rename = "turnstile", skip_serializing_if = "Option::is_none")]
    pub turnstile: Option<Box<crate::models::CaptchaConfigTurnstile>>,
}

impl CaptchaConfig {
    /// Methods to verify a captcha
    pub fn new() -> CaptchaConfig {
        CaptchaConfig {
            hcaptcha: None,
            turnstile: None,
        }
    }
}


