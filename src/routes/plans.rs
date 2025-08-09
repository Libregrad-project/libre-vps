use axum::{extract::State, Json};
use serde_json::json;
use std::sync::Arc;

use crate::services::provider::ProviderService;

// D-Provider

pub async fn list_intel_xeon(
    State(provider): State<Arc<ProviderService>>,
) -> Json<serde_json::Value> {
    match provider.list_intel_xeon().await {
        Ok(data) => Json(data),
        Err(err) => Json(json!({
            "error": err.to_string()
        })),
    }
}

pub async fn list_amd_epyc(
    State(provider): State<Arc<ProviderService>>,
) -> Json<serde_json::Value> {
    match provider.list_amd_epyc().await {
        Ok(data) => Json(data),
        Err(err) => Json(json!({
            "error": err.to_string()
        })),
    }
}

// C - Provider

pub async fn c_list_intel_xeon(
    State(provider): State<Arc<ProviderService>>,
) -> Json<serde_json::Value> {
    match provider.c_list_intel_xeon().await {
        Ok(data) => Json(data),
        Err(err) => Json(json!({
            "error": err.to_string()
        })),
    }
}

pub async fn c_list_amd_epyc(
    State(provider): State<Arc<ProviderService>>,
) -> Json<serde_json::Value> {
    match provider.c_list_amd_epyc().await {
        Ok(data) => Json(data),
        Err(err) => Json(json!({
            "error": err.to_string()
        })),
    }
}