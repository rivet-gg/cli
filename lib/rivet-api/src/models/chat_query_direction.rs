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
pub enum ChatQueryDirection {
    #[serde(rename = "before")]
    Before,
    #[serde(rename = "after")]
    After,
    #[serde(rename = "before_and_after")]
    BeforeAndAfter,

}

impl ToString for ChatQueryDirection {
    fn to_string(&self) -> String {
        match self {
            Self::Before => String::from("before"),
            Self::After => String::from("after"),
            Self::BeforeAndAfter => String::from("before_and_after"),
        }
    }
}

impl Default for ChatQueryDirection {
    fn default() -> ChatQueryDirection {
        Self::Before
    }
}



