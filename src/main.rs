use actix_cors::Cors;
use actix_web::{middleware::DefaultHeaders, web, App, HttpServer};
use redis::Client;
use tokio;
use aws_config::{meta::region::RegionProviderChain, BehaviorVersion};
use aws_sdk_s3::Client as S3Client;

use crate::routes::init_routes;

mod storage;
mod routes;
mod services;
mod models;
mod controllers;

use services::export::ExportService;
#[derive(Clone)]
pub struct AppState {
    redis_client: Client,
    export_service: ExportService
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let valkey_port = std::env::var("VALKEY_PORT").unwrap();
    let valkey_host = std::env::var("VALKEY_HOST").unwrap();
    let url = format!("redis://{valkey_host}:{valkey_port}");

    let redis_client = Client::open(url)?;

     // Config S3 (ejemplo con MinIO en Docker)
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let shared_config = aws_config::defaults(BehaviorVersion::latest())
        .region(region_provider)
        .endpoint_url("{}:{}") // MinIO en Docker
        .load()
        .await;

    let s3_client = S3Client::new(&shared_config);

    let export_service = ExportService::new(
        "chat-export.parquet".into(),
        Some(s3_client),
        Some("my-bucket".into()),
    );

    let state = AppState {
        redis_client,
        export_service
    };

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin() 
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);
        App::new()
            .app_data(web::Data::new(state.clone()))

            .wrap(cors)
            .wrap(
                DefaultHeaders::new()
                    .add(("X-Content-Type-Options", "nosniff"))
                    .add(("X-Frame-Options", "DENY"))
                    .add(("X-XSS-Protection", "1; mode=block"))
                    .add(("Strict-Transport-Security", "max-age=31536000; includeSubDomains"))
                    .add(("Referrer-Policy", "no-referrer"))
                    .add(("Permissions-Policy", "geolocation=(), microphone=()"))
            )
            .service(web::scope("/api/v1").configure(init_routes))
    })
    .bind(("0.0.0.0", 8080))?
    .max_connections(2000)
    .workers(4)
    .run()
    .await?;

    Ok(())
}
