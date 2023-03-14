/*
 * Rivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ChatSimpleTopic : Represents a topic of the given chat thread without the associated handles for the topic. 



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChatSimpleTopic {
    #[serde(rename = "direct", skip_serializing_if = "Option::is_none")]
    pub direct: Option<Box<crate::models::ChatSimpleTopicDirect>>,
    #[serde(rename = "group", skip_serializing_if = "Option::is_none")]
    pub group: Option<Box<crate::models::ChatSimpleTopicGroup>>,
    #[serde(rename = "party", skip_serializing_if = "Option::is_none")]
    pub party: Option<Box<crate::models::ChatSimpleTopicParty>>,
}

impl ChatSimpleTopic {
    /// Represents a topic of the given chat thread without the associated handles for the topic. 
    pub fn new() -> ChatSimpleTopic {
        ChatSimpleTopic {
            direct: None,
            group: None,
            party: None,
        }
    }
}


