/*
 * Rivet API EE
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GroupPublicity : The current publicity value for the given group.

/// The current publicity value for the given group.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum GroupPublicity {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "closed")]
    Closed,

}

impl ToString for GroupPublicity {
    fn to_string(&self) -> String {
        match self {
            Self::Open => String::from("open"),
            Self::Closed => String::from("closed"),
        }
    }
}

impl Default for GroupPublicity {
    fn default() -> GroupPublicity {
        Self::Open
    }
}




