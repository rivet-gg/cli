/*
 * Rivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CommonsPartyPublicityLevel {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "view")]
    View,
    #[serde(rename = "join")]
    Join,

}

impl ToString for CommonsPartyPublicityLevel {
    fn to_string(&self) -> String {
        match self {
            Self::None => String::from("none"),
            Self::View => String::from("view"),
            Self::Join => String::from("join"),
        }
    }
}

impl Default for CommonsPartyPublicityLevel {
    fn default() -> CommonsPartyPublicityLevel {
        Self::None
    }
}



