use leptos::logging::log;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ApiData {
    pub id: String,
    pub name: String,
}

pub async fn get_api_data(fetch_id: String) -> ApiData {
    log!("run get_api_data with id: {fetch_id}");

    ApiData {
        id: fetch_id.clone(),
        name: format!("Name for {fetch_id}"),
    }
}

#[derive(Clone, Debug)]
pub enum ClientError {
    NotFound,
    InternalError,
}

#[derive(Clone)]
pub struct Client;

impl Client {
    pub fn new() -> Self {
        Client
    }

    pub async fn fetch_data(&self, id: String) -> Result<ApiData, ClientError> {
        Ok(ApiData {
            id: id.clone(),
            name: format!("Name for {id}"),
        })
    }
}
