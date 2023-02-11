/*
 * Rivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CloudGamesNamespacesUpdateNamespaceCdnAuthUserInput {
    /// A bcrypt encrypted password. An error is returned if the given string is not properly encrypted.
    #[serde(rename = "password")]
    pub password: String,
    /// A user name.
    #[serde(rename = "user")]
    pub user: String,
}

impl CloudGamesNamespacesUpdateNamespaceCdnAuthUserInput {
    pub fn new(password: String, user: String) -> CloudGamesNamespacesUpdateNamespaceCdnAuthUserInput {
        CloudGamesNamespacesUpdateNamespaceCdnAuthUserInput {
            password,
            user,
        }
    }
}

