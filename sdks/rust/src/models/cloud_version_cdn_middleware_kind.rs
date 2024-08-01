/*
 * Rivet API EE
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CloudVersionCdnMiddlewareKind {
    #[serde(rename = "custom_headers", skip_serializing_if = "Option::is_none")]
    pub custom_headers: Option<Box<crate::models::CloudVersionCdnCustomHeadersMiddleware>>,
}

impl CloudVersionCdnMiddlewareKind {
    pub fn new() -> CloudVersionCdnMiddlewareKind {
        CloudVersionCdnMiddlewareKind {
            custom_headers: None,
        }
    }
}


