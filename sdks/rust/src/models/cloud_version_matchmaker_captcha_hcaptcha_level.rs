/*
 * Rivet API EE
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CloudVersionMatchmakerCaptchaHcaptchaLevel : **Deprecated** How hard a captcha should be.

/// **Deprecated** How hard a captcha should be.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CloudVersionMatchmakerCaptchaHcaptchaLevel {
    #[serde(rename = "easy")]
    Easy,
    #[serde(rename = "moderate")]
    Moderate,
    #[serde(rename = "difficult")]
    Difficult,
    #[serde(rename = "always_on")]
    AlwaysOn,

}

impl ToString for CloudVersionMatchmakerCaptchaHcaptchaLevel {
    fn to_string(&self) -> String {
        match self {
            Self::Easy => String::from("easy"),
            Self::Moderate => String::from("moderate"),
            Self::Difficult => String::from("difficult"),
            Self::AlwaysOn => String::from("always_on"),
        }
    }
}

impl Default for CloudVersionMatchmakerCaptchaHcaptchaLevel {
    fn default() -> CloudVersionMatchmakerCaptchaHcaptchaLevel {
        Self::Easy
    }
}




