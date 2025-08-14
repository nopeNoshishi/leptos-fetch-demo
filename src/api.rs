//! This module provides the demo API client and data structures for fetching data.

use leptos::logging::log;
use serde::{Deserialize, Serialize};

/// Represents the API data structure.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ApiData {
    /// The unique identifier for the data.
    pub id: String,

    /// The name associated with the data.
    pub name: String,
}

/// Fetches API data based on the provided ID.
pub async fn get_api_data(fetch_id: String) -> ApiData {
    log!("run get_api_data with id: {fetch_id}");

    ApiData {
        id: fetch_id.clone(),
        name: format!("Name for {fetch_id}"),
    }
}

/// Represents a client for making API requests.
#[derive(Clone, Debug)]
pub enum ClientError {
    /// Represents a not found error.
    NotFound,

    /// Represents an internal server error.
    InternalError,
}

/// Represents a client for making API requests.
#[derive(Clone)]
pub struct Client;

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

impl Client {
    /// Constructs a new API client.
    pub fn new() -> Self {
        Client
    }

    /// Fetches API data based on the provided ID.
    pub async fn fetch_data(&self, id: String) -> Result<ApiData, ClientError> {
        log!("run Client::get_api_data with id: {id}");

        Ok(ApiData {
            id: id.clone(),
            name: format!("Name for {id}"),
        })
    }
}
