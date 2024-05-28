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
pub struct EeCloudDestroyedLobbiesByExitCodeDataSet {
    #[serde(rename = "destroyed_lobby_count")]
    pub destroyed_lobby_count: Vec<i64>,
    #[serde(rename = "exit_code")]
    pub exit_code: Vec<i32>,
    #[serde(rename = "ts")]
    pub ts: Vec<i64>,
}

impl EeCloudDestroyedLobbiesByExitCodeDataSet {
    pub fn new(destroyed_lobby_count: Vec<i64>, exit_code: Vec<i32>, ts: Vec<i64>) -> EeCloudDestroyedLobbiesByExitCodeDataSet {
        EeCloudDestroyedLobbiesByExitCodeDataSet {
            destroyed_lobby_count,
            exit_code,
            ts,
        }
    }
}


