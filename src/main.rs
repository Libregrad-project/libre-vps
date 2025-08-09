mod routes;
mod services;

use axum::{routing::get, Router};
use services::provider::ProviderService;
use std::sync::Arc;
use std::env;
use axum::serve;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {

    dotenvy::dotenv().ok();
    let d_base_url = env::var("D_PROVIDER_BASE_URL")
        .unwrap_or_else(|_| "https://backend.datalix.de/v1".to_string());
    let d_api_key = env::var("D_PROVIDER_API_KEY")
        .expect("Missing D_PROVIDER_API_KEY");

    let c_base_url = env::var("C_PROVIDER_BASE_URL")
        .unwrap_or_else(|_| "https://another.api.com/v1".to_string());
    let c_api_key = env::var("C_PROVIDER_API_KEY")
        .expect("Missing C_PROVIDER_API_KEY");

    let provider_service = Arc::new(ProviderService::new(
        d_base_url,
        d_api_key,
        c_base_url,
        c_api_key,
    ));

    // Build routes
    let app = Router::new()
    // D-Routes
        .route(
            "/providers/d/intel-xeon",
            get(routes::plans::list_intel_xeon),
        )
        .route(
            "/providers/d/amd-epyc",
            get(routes::plans::list_amd_epyc),
        )
        .route(
            "/providers/d/intel-xeon/information/{packetid}",
            get(routes::packages::intel_xeon_plan_information),
        )
        .route(
            "/providers/d/amd-epyc/information/{packetid}",
            get(routes::packages::amd_epyc_plan_information),
        )
        .route(
            "/providers/d/intel-xeon/information/{packetid}/os",
            get(routes::os::intel_xeon_os_information),
        )
        .route(
            "/providers/d/amd-epyc/information/{packetid}/os",
            get(routes::os::amd_epyc_os_information),
        )

        // C-Routes
        .route(
            "/providers/c/intel-xeon",
            get(routes::plans::c_list_intel_xeon),
        )
        .route(
            "/providers/c/amd-epyc",
            get(routes::plans::c_list_amd_epyc),
        )
        .route(
            "/providers/c/intel-xeon/information/{packetid}",
            get(routes::packages::c_intel_xeon_plan_information),
        )
        .route(
            "/providers/c/amd-epyc/information/{packetid}",
            get(routes::packages::c_amd_epyc_plan_information),
        )
        .route(
            "/providers/c/intel-xeon/information/{packetid}/os",
            get(routes::os::c_intel_xeon_os_information),
        )
        .route(
            "/providers/c/amd-epyc/information/{packetid}/os",
            get(routes::os::c_amd_epyc_os_information),
        )

        .with_state(provider_service);

    // Start server
    println!("Server running at http://127.0.0.1:3000");
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    serve(listener, app).await.unwrap();
}