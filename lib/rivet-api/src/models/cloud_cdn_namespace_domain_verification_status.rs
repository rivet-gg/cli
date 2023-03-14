/*
 * Rivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CloudCdnNamespaceDomainVerificationStatus : A value denoting the status of a CDN domain's verification status.

/// A value denoting the status of a CDN domain's verification status.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum CloudCdnNamespaceDomainVerificationStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "failed")]
    Failed,

}

impl ToString for CloudCdnNamespaceDomainVerificationStatus {
    fn to_string(&self) -> String {
        match self {
            Self::Active => String::from("active"),
            Self::Pending => String::from("pending"),
            Self::Failed => String::from("failed"),
        }
    }
}

impl Default for CloudCdnNamespaceDomainVerificationStatus {
    fn default() -> CloudCdnNamespaceDomainVerificationStatus {
        Self::Active
    }
}




