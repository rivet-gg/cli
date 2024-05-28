/*
 * Rivet API EE
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IdentityExternalLinks : External links for an identity.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct IdentityExternalLinks {
    /// A link to this identity's profile page.
    #[serde(rename = "profile")]
    pub profile: String,
    /// A link to the Rivet settings page.
    #[serde(rename = "settings", skip_serializing_if = "Option::is_none")]
    pub settings: Option<String>,
}

impl IdentityExternalLinks {
    /// External links for an identity.
    pub fn new(profile: String) -> IdentityExternalLinks {
        IdentityExternalLinks {
            profile,
            settings: None,
        }
    }
}


