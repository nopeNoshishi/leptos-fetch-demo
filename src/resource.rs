//! This module provides a wrapper around the local resource that fetches data using [`leptos_fetch::QueryClient`].

mod wrapper_local_resource;

pub use wrapper_local_resource::{WrapperLocalResource, data_resource};
