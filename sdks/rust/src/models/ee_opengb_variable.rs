/*
 * Rivet API EE
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// EeOpengbVariable : An environment variable patch. One of these properties must be set.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct EeOpengbVariable {
    #[serde(rename = "delete", skip_serializing_if = "Option::is_none")]
    pub delete: Option<bool>,
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    #[serde(rename = "text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

impl EeOpengbVariable {
    /// An environment variable patch. One of these properties must be set.
    pub fn new() -> EeOpengbVariable {
        EeOpengbVariable {
            delete: None,
            secret: None,
            text: None,
        }
    }
}


