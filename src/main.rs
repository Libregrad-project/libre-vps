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
    let api_key = env::var("DATALIX_API_KEY")
        .expect("Missing DATALIX_API_KEY in environment");

    // Create the service
    let provider_service = Arc::new(ProviderService::new(api_key));

    // Build routes
    let app = Router::new()
        .route(
            "/providers/intel-xeon",
            get(routes::plans::list_intel_xeon),
        )
        .route(
            "/providers/amd-epyc",
            get(routes::plans::list_amd_epyc),
        )
        .route(
            "/providers/intel-xeon/information/{packetid}",
            get(routes::packages::intel_xeon_plan_information),
        )
        .route(
            "/providers/amd-epyc/information/{packetid}",
            get(routes::packages::amd_epyc_plan_information),
        )
        .route(
            "/providers/intel-xeon/information/{packetid}/os",
            get(routes::os::intel_xeon_os_information),
        )
        .route(
            "/providers/amd-epyc/information/{packetid}/os",
            get(routes::os::amd_epyc_os_information),
        )
        .with_state(provider_service);

    // Start server
    println!("Server running at http://127.0.0.1:3000");
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    serve(listener, app).await.unwrap();
}