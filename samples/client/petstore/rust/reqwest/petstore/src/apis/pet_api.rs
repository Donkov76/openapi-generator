/*
 * OpenAPI Petstore
 *
 * This is a sample server Petstore server. For this sample, you can use the api key `special-key` to test the authorization filters.
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method `add_pet`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddPetError {
    Status405(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `delete_pet`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeletePetError {
    Status400(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `find_pets_by_status`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FindPetsByStatusError {
    Status400(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `find_pets_by_tags`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FindPetsByTagsError {
    Status400(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_pet_by_id`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetPetByIdError {
    Status400(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `update_pet`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdatePetError {
    Status400(),
    Status404(),
    Status405(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `update_pet_with_form`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdatePetWithFormError {
    Status405(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `upload_file`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UploadFileError {
    UnknownValue(serde_json::Value),
}


pub fn add_pet(configuration: &configuration::Configuration, body: crate::models::Pet) -> Result<(), Error<AddPetError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/pet", configuration.base_path);
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = configuration.oauth_access_token {
        local_var_req_builder = req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let mut local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if local_var_status.is_success() {
        Ok(())
    } else {
        let local_var_entity: Option<AddPetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { local_var_status, local_var_content, local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub fn delete_pet(configuration: &configuration::Configuration, pet_id: i64, api_key: Option<&str>) -> Result<(), Error<DeletePetError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/pet/{petId}", configuration.base_path, petId=pet_id);
    let mut local_var_req_builder = local_var_client.delete(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = api_key {
        local_var_req_builder = local_var_req_builder.header("api_key", local_var_param_value.to_string());
    }
    if let Some(ref local_var_token) = configuration.oauth_access_token {
        local_var_req_builder = req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let mut local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if local_var_status.is_success() {
        Ok(())
    } else {
        let local_var_entity: Option<DeletePetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { local_var_status, local_var_content, local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Multiple status values can be provided with comma separated strings
pub fn find_pets_by_status(configuration: &configuration::Configuration, status: Vec<String>) -> Result<Vec<crate::models::Pet>, Error<FindPetsByStatusError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/pet/findByStatus", configuration.base_path);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("status", &status.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]);
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = configuration.oauth_access_token {
        local_var_req_builder = req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let mut local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if local_var_status.is_success() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<FindPetsByStatusError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { local_var_status, local_var_content, local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Multiple tags can be provided with comma separated strings. Use tag1, tag2, tag3 for testing.
pub fn find_pets_by_tags(configuration: &configuration::Configuration, tags: Vec<String>) -> Result<Vec<crate::models::Pet>, Error<FindPetsByTagsError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/pet/findByTags", configuration.base_path);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("tags", &tags.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]);
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = configuration.oauth_access_token {
        local_var_req_builder = req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let mut local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if local_var_status.is_success() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<FindPetsByTagsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { local_var_status, local_var_content, local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns a single pet
pub fn get_pet_by_id(configuration: &configuration::Configuration, pet_id: i64) -> Result<crate::models::Pet, Error<GetPetByIdError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/pet/{petId}", configuration.base_path, petId=pet_id);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("api_key", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let mut local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if local_var_status.is_success() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetPetByIdError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { local_var_status, local_var_content, local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub fn update_pet(configuration: &configuration::Configuration, body: crate::models::Pet) -> Result<(), Error<UpdatePetError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/pet", configuration.base_path);
    let mut local_var_req_builder = local_var_client.put(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = configuration.oauth_access_token {
        local_var_req_builder = req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let mut local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if local_var_status.is_success() {
        Ok(())
    } else {
        let local_var_entity: Option<UpdatePetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { local_var_status, local_var_content, local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub fn update_pet_with_form(configuration: &configuration::Configuration, pet_id: i64, name: Option<&str>, status: Option<&str>) -> Result<(), Error<UpdatePetWithFormError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/pet/{petId}", configuration.base_path, petId=pet_id);
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = configuration.oauth_access_token {
        local_var_req_builder = req_builder.bearer_auth(local_var_token.to_owned());
    };
    let mut local_var_form_params = std::collections::HashMap::new();
    if let Some(local_var_param_value) = name {
        local_var_form_params.insert("name", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = status {
        local_var_form_params.insert("status", local_var_param_value.to_string());
    }
    local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

    let local_var_req = local_var_req_builder.build()?;
    let mut local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if local_var_status.is_success() {
        Ok(())
    } else {
        let local_var_entity: Option<UpdatePetWithFormError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { local_var_status, local_var_content, local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub fn upload_file(configuration: &configuration::Configuration, pet_id: i64, additional_metadata: Option<&str>, file: Option<std::path::PathBuf>) -> Result<crate::models::ApiResponse, Error<UploadFileError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/pet/{petId}/uploadImage", configuration.base_path, petId=pet_id);
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = configuration.oauth_access_token {
        local_var_req_builder = req_builder.bearer_auth(local_var_token.to_owned());
    };
    let mut local_var_form = reqwest::multipart::Form::new();
    if let Some(local_var_param_value) = additional_metadata {
        local_var_form = local_var_form.text("additionalMetadata", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = file {
        local_var_form = local_var_form.file("file", local_var_param_value)?;
    }
    local_var_req_builder = local_var_req_builder.multipart(form);

    let local_var_req = local_var_req_builder.build()?;
    let mut local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if local_var_status.is_success() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UploadFileError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { local_var_status, local_var_content, local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

