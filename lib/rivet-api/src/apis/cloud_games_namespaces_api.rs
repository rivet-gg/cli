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


/// struct for typed errors of method [`namespaces_add_namespace_domain`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NamespacesAddNamespaceDomainError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`namespaces_create_game_namespace`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NamespacesCreateGameNamespaceError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`namespaces_create_game_namespace_token_development`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NamespacesCreateGameNamespaceTokenDevelopmentError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`namespaces_create_game_namespace_token_public`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NamespacesCreateGameNamespaceTokenPublicError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`namespaces_get_game_namespace_by_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NamespacesGetGameNamespaceByIdError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`namespaces_remove_namespace_cdn_auth_user`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NamespacesRemoveNamespaceCdnAuthUserError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`namespaces_remove_namespace_domain`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NamespacesRemoveNamespaceDomainError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`namespaces_set_namespace_cdn_auth_type`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NamespacesSetNamespaceCdnAuthTypeError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`namespaces_toggle_namespace_domain_public_auth`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NamespacesToggleNamespaceDomainPublicAuthError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`namespaces_update_game_namespace_matchmaker_config`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NamespacesUpdateGameNamespaceMatchmakerConfigError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`namespaces_update_game_namespace_version`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NamespacesUpdateGameNamespaceVersionError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`namespaces_update_namespace_cdn_auth_user`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NamespacesUpdateNamespaceCdnAuthUserError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`namespaces_validate_game_namespace`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NamespacesValidateGameNamespaceError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`namespaces_validate_game_namespace_matchmaker_config`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NamespacesValidateGameNamespaceMatchmakerConfigError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`namespaces_validate_game_namespace_token_development`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NamespacesValidateGameNamespaceTokenDevelopmentError {
    UnknownValue(serde_json::Value),
}


/// Adds a domain to the given game namespace.
pub async fn namespaces_add_namespace_domain(configuration: &configuration::Configuration, game_id: &str, namespace_id: &str, cloud_games_add_namespace_domain_input: crate::models::CloudGamesAddNamespaceDomainInput) -> Result<(), Error<NamespacesAddNamespaceDomainError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/games/{game_id}/namespaces/{namespace_id}/domains", local_var_configuration.base_path, game_id=crate::apis::urlencode(game_id), namespace_id=crate::apis::urlencode(namespace_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&cloud_games_add_namespace_domain_input);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<NamespacesAddNamespaceDomainError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Creates a new namespace for the given game.
pub async fn namespaces_create_game_namespace(configuration: &configuration::Configuration, game_id: &str, cloud_games_create_game_namespace_input: crate::models::CloudGamesCreateGameNamespaceInput) -> Result<crate::models::CloudGamesCreateGameNamespaceOutput, Error<NamespacesCreateGameNamespaceError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/games/{game_id}/namespaces", local_var_configuration.base_path, game_id=crate::apis::urlencode(game_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&cloud_games_create_game_namespace_input);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<NamespacesCreateGameNamespaceError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Creates a development token for the given namespace.
pub async fn namespaces_create_game_namespace_token_development(configuration: &configuration::Configuration, game_id: &str, namespace_id: &str, cloud_games_create_game_namespace_token_development_input: crate::models::CloudGamesCreateGameNamespaceTokenDevelopmentInput) -> Result<crate::models::CloudGamesCreateGameNamespaceTokenDevelopmentOutput, Error<NamespacesCreateGameNamespaceTokenDevelopmentError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/games/{game_id}/namespaces/{namespace_id}/tokens/development", local_var_configuration.base_path, game_id=crate::apis::urlencode(game_id), namespace_id=crate::apis::urlencode(namespace_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&cloud_games_create_game_namespace_token_development_input);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<NamespacesCreateGameNamespaceTokenDevelopmentError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Creates a public token for the given namespace.
pub async fn namespaces_create_game_namespace_token_public(configuration: &configuration::Configuration, game_id: &str, namespace_id: &str) -> Result<crate::models::CloudGamesCreateGameNamespaceTokenPublicOutput, Error<NamespacesCreateGameNamespaceTokenPublicError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/games/{game_id}/namespaces/{namespace_id}/tokens/public", local_var_configuration.base_path, game_id=crate::apis::urlencode(game_id), namespace_id=crate::apis::urlencode(namespace_id));
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
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<NamespacesCreateGameNamespaceTokenPublicError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Gets a game namespace by namespace ID.
pub async fn namespaces_get_game_namespace_by_id(configuration: &configuration::Configuration, game_id: &str, namespace_id: &str) -> Result<crate::models::CloudGamesGetGameNamespaceByIdOutput, Error<NamespacesGetGameNamespaceByIdError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/games/{game_id}/namespaces/{namespace_id}", local_var_configuration.base_path, game_id=crate::apis::urlencode(game_id), namespace_id=crate::apis::urlencode(namespace_id));
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
        let local_var_entity: Option<NamespacesGetGameNamespaceByIdError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Removes an authenticated user from the given game namespace.
pub async fn namespaces_remove_namespace_cdn_auth_user(configuration: &configuration::Configuration, game_id: &str, namespace_id: &str, user: &str) -> Result<(), Error<NamespacesRemoveNamespaceCdnAuthUserError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/games/{game_id}/namespaces/{namespace_id}/auth-user/{user}", local_var_configuration.base_path, game_id=crate::apis::urlencode(game_id), namespace_id=crate::apis::urlencode(namespace_id), user=crate::apis::urlencode(user));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

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
        let local_var_entity: Option<NamespacesRemoveNamespaceCdnAuthUserError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Removes a domain from the given game namespace.
pub async fn namespaces_remove_namespace_domain(configuration: &configuration::Configuration, game_id: &str, namespace_id: &str, domain: &str) -> Result<(), Error<NamespacesRemoveNamespaceDomainError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/games/{game_id}/namespaces/{namespace_id}/domains/{domain}", local_var_configuration.base_path, game_id=crate::apis::urlencode(game_id), namespace_id=crate::apis::urlencode(namespace_id), domain=crate::apis::urlencode(domain));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

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
        let local_var_entity: Option<NamespacesRemoveNamespaceDomainError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Updates the CDN authentication type of the given game namesapce.
pub async fn namespaces_set_namespace_cdn_auth_type(configuration: &configuration::Configuration, game_id: &str, namespace_id: &str, cloud_games_set_namespace_cdn_auth_type_input: crate::models::CloudGamesSetNamespaceCdnAuthTypeInput) -> Result<(), Error<NamespacesSetNamespaceCdnAuthTypeError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/games/{game_id}/namespaces/{namespace_id}/cdn-auth", local_var_configuration.base_path, game_id=crate::apis::urlencode(game_id), namespace_id=crate::apis::urlencode(namespace_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&cloud_games_set_namespace_cdn_auth_type_input);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<NamespacesSetNamespaceCdnAuthTypeError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Toggles whether or not to allow authentication based on domain for the given game namesapce.
pub async fn namespaces_toggle_namespace_domain_public_auth(configuration: &configuration::Configuration, game_id: &str, namespace_id: &str, cloud_games_toggle_namespace_domain_public_auth_input: crate::models::CloudGamesToggleNamespaceDomainPublicAuthInput) -> Result<(), Error<NamespacesToggleNamespaceDomainPublicAuthError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/games/{game_id}/namespaces/{namespace_id}/domain-public-auth", local_var_configuration.base_path, game_id=crate::apis::urlencode(game_id), namespace_id=crate::apis::urlencode(namespace_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&cloud_games_toggle_namespace_domain_public_auth_input);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<NamespacesToggleNamespaceDomainPublicAuthError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Updates matchmaker config for the given game namespace.
pub async fn namespaces_update_game_namespace_matchmaker_config(configuration: &configuration::Configuration, game_id: &str, namespace_id: &str, cloud_games_update_game_namespace_matchmaker_config_input: crate::models::CloudGamesUpdateGameNamespaceMatchmakerConfigInput) -> Result<(), Error<NamespacesUpdateGameNamespaceMatchmakerConfigError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/games/{game_id}/namespaces/{namespace_id}/mm-config", local_var_configuration.base_path, game_id=crate::apis::urlencode(game_id), namespace_id=crate::apis::urlencode(namespace_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&cloud_games_update_game_namespace_matchmaker_config_input);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<NamespacesUpdateGameNamespaceMatchmakerConfigError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Updates the version of a game namespace.
pub async fn namespaces_update_game_namespace_version(configuration: &configuration::Configuration, game_id: &str, namespace_id: &str, cloud_games_update_game_namespace_version_input: crate::models::CloudGamesUpdateGameNamespaceVersionInput) -> Result<(), Error<NamespacesUpdateGameNamespaceVersionError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/games/{game_id}/namespaces/{namespace_id}/version", local_var_configuration.base_path, game_id=crate::apis::urlencode(game_id), namespace_id=crate::apis::urlencode(namespace_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&cloud_games_update_game_namespace_version_input);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<NamespacesUpdateGameNamespaceVersionError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Adds an authenticated user to the given game namespace.
pub async fn namespaces_update_namespace_cdn_auth_user(configuration: &configuration::Configuration, game_id: &str, namespace_id: &str, cloud_games_update_namespace_cdn_auth_user_input: crate::models::CloudGamesUpdateNamespaceCdnAuthUserInput) -> Result<(), Error<NamespacesUpdateNamespaceCdnAuthUserError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/games/{game_id}/namespaces/{namespace_id}/auth-user", local_var_configuration.base_path, game_id=crate::apis::urlencode(game_id), namespace_id=crate::apis::urlencode(namespace_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&cloud_games_update_namespace_cdn_auth_user_input);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<NamespacesUpdateNamespaceCdnAuthUserError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Validates information used to create a new game namespace.
pub async fn namespaces_validate_game_namespace(configuration: &configuration::Configuration, game_id: &str, cloud_games_validate_game_namespace_input: crate::models::CloudGamesValidateGameNamespaceInput) -> Result<crate::models::CloudGamesValidateGameNamespaceOutput, Error<NamespacesValidateGameNamespaceError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/games/{game_id}/namespaces/validate", local_var_configuration.base_path, game_id=crate::apis::urlencode(game_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&cloud_games_validate_game_namespace_input);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<NamespacesValidateGameNamespaceError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Validates information used to update a game namespace's matchmaker config.
pub async fn namespaces_validate_game_namespace_matchmaker_config(configuration: &configuration::Configuration, game_id: &str, namespace_id: &str, cloud_games_validate_game_namespace_matchmaker_config_input: crate::models::CloudGamesValidateGameNamespaceMatchmakerConfigInput) -> Result<crate::models::CloudGamesValidateGameNamespaceMatchmakerConfigOutput, Error<NamespacesValidateGameNamespaceMatchmakerConfigError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/games/{game_id}/namespaces/{namespace_id}/mm-config/validate", local_var_configuration.base_path, game_id=crate::apis::urlencode(game_id), namespace_id=crate::apis::urlencode(namespace_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&cloud_games_validate_game_namespace_matchmaker_config_input);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<NamespacesValidateGameNamespaceMatchmakerConfigError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Validates information used to create a new game namespace development token.
pub async fn namespaces_validate_game_namespace_token_development(configuration: &configuration::Configuration, game_id: &str, namespace_id: &str, cloud_games_validate_game_namespace_token_development_input: crate::models::CloudGamesValidateGameNamespaceTokenDevelopmentInput) -> Result<crate::models::CloudGamesValidateGameNamespaceTokenDevelopmentOutput, Error<NamespacesValidateGameNamespaceTokenDevelopmentError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/games/{game_id}/namespaces/{namespace_id}/tokens/development/validate", local_var_configuration.base_path, game_id=crate::apis::urlencode(game_id), namespace_id=crate::apis::urlencode(namespace_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&cloud_games_validate_game_namespace_token_development_input);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<NamespacesValidateGameNamespaceTokenDevelopmentError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

