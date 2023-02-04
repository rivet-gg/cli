/*
 * Rivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method [`avatars_service_period_complete_custom_avatar_upload`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AvatarsServicePeriodCompleteCustomAvatarUploadError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`avatars_service_period_list_game_custom_avatars`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AvatarsServicePeriodListGameCustomAvatarsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`avatars_service_period_prepare_custom_avatar_upload`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AvatarsServicePeriodPrepareCustomAvatarUploadError {
    UnknownValue(serde_json::Value),
}


/// Completes a custom avatar image upload. Must be called after the file upload process completes.
pub async fn avatars_service_period_complete_custom_avatar_upload(configuration: &configuration::Configuration, game_id: &str, upload_id: &str) -> Result<(), Error<AvatarsServicePeriodCompleteCustomAvatarUploadError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/games/{game_id}/avatar-upload/{upload_id}/complete", local_var_configuration.base_path, game_id=crate::apis::urlencode(game_id), upload_id=crate::apis::urlencode(upload_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<AvatarsServicePeriodCompleteCustomAvatarUploadError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Lists custom avatars for the given game.
pub async fn avatars_service_period_list_game_custom_avatars(configuration: &configuration::Configuration, game_id: &str) -> Result<crate::models::CloudGamesListGameCustomAvatarsOutput, Error<AvatarsServicePeriodListGameCustomAvatarsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/games/{game_id}/avatars", local_var_configuration.base_path, game_id=crate::apis::urlencode(game_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<AvatarsServicePeriodListGameCustomAvatarsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Prepares a custom avatar image upload. Complete upload with `rivet.api.cloud#CompleteCustomAvatarUpload`.
pub async fn avatars_service_period_prepare_custom_avatar_upload(configuration: &configuration::Configuration, game_id: &str, cloud_games_prepare_custom_avatar_upload_input: crate::models::CloudGamesPrepareCustomAvatarUploadInput) -> Result<crate::models::CloudGamesPrepareCustomAvatarUploadOutput, Error<AvatarsServicePeriodPrepareCustomAvatarUploadError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/games/{game_id}/prepare", local_var_configuration.base_path, game_id=crate::apis::urlencode(game_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&cloud_games_prepare_custom_avatar_upload_input);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<AvatarsServicePeriodPrepareCustomAvatarUploadError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}
