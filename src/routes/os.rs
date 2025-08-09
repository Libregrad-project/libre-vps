use axum::{extract::{State, Path}, Json};
use serde_json::json;
use std::sync::Arc;
use crate::services::provider::ProviderService;

// D

pub async fn intel_xeon_plan_information(
    State(provider): State<Arc<ProviderService>>,
    Path(packetid): Path<String>,
) -> Json<serde_json::Value> {
    match provider.list_intel_xeon_information(&packetid).await {
        Ok(data) => Json(data),
        Err(err) => Json(json!({"error": err.to_string()})),
    }
}

pub async fn intel_xeon_os_information(
    State(provider): State<Arc<ProviderService>>,
    Path(packetid): Path<String>,
) -> Json<serde_json::Value> {
    match provider.list_intel_xeon_os_information(&packetid).await {
        Ok(data) => Json(data),
        Err(err) => Json(json!({"error": err.to_string()})),
    }
}

pub async fn amd_epyc_plan_information(
    State(provider): State<Arc<ProviderService>>,
    Path(packetid): Path<String>,
) -> Json<serde_json::Value> {
    match provider.list_intel_xeon_information(&packetid).await {
        Ok(data) => Json(data),
        Err(err) => Json(json!({"error": err.to_string()})),
    }
}

pub async fn amd_epyc_os_information(
    State(provider): State<Arc<ProviderService>>,
    Path(packetid): Path<String>,
) -> Json<serde_json::Value> {
    match provider.list_amd_epyc_os_information(&packetid).await {
        Ok(data) => Json(data),
        Err(err) => Json(json!({"error": err.to_string()})),
    }
}

// C

pub async fn c_intel_xeon_plan_information(
    State(provider): State<Arc<ProviderService>>,
    Path(packetid): Path<String>,
) -> Json<serde_json::Value> {
    match provider.c_list_intel_xeon_information(&packetid).await {
        Ok(data) => Json(data),
        Err(err) => Json(json!({"error": err.to_string()})),
    }
}

pub async fn c_intel_xeon_os_information(
    State(provider): State<Arc<ProviderService>>,
    Path(packetid): Path<String>,
) -> Json<serde_json::Value> {
    match provider.c_list_intel_xeon_os_information(&packetid).await {
        Ok(data) => Json(data),
        Err(err) => Json(json!({"error": err.to_string()})),
    }
}

pub async fn c_amd_epyc_plan_information(
    State(provider): State<Arc<ProviderService>>,
    Path(packetid): Path<String>,
) -> Json<serde_json::Value> {
    match provider.c_list_amd_epyc_information(&packetid).await {
        Ok(data) => Json(data),
        Err(err) => Json(json!({"error": err.to_string()})),
    }
}

pub async fn c_amd_epyc_os_information(
    State(provider): State<Arc<ProviderService>>,
    Path(packetid): Path<String>,
) -> Json<serde_json::Value> {
    match provider.c_list_amd_epyc_os_information(&packetid).await {
        Ok(data) => Json(data),
        Err(err) => Json(json!({"error": err.to_string()})),
    }
}