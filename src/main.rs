use actix_cors::Cors;
use actix_web::{App, HttpServer, middleware::DefaultHeaders, web};
use tokio;

use crate::{routes::init_routes, services::{history_service::HistoryService, message_service::MessageService, trigger_service::TriggerService}};

mod config;
mod controllers;
mod models;
mod routes;
mod services;
mod storage;
mod workers;

use services::export_service::ExportService;
#[derive(Clone)]
pub struct AppState {
    pub message_service: MessageService,
    pub export_service: ExportService,
    pub history_service: HistoryService
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let _ = unsafe { std::env::set_var("RUST_BACKTRACE", "1") };
    env_logger::init();
    let redis_config = config::valkey::ValkeyConfig::load_env_or_default();
    let redis_client = redis_config.create_client().await?;

    let s3_config = config::s3::S3Config::from_env_or_default();
    let s3_client = s3_config.create_client().await;

    let export_service = ExportService::new(
        "/tmp/messages.parquet".into(),
        s3_client.clone(),
        redis_client.clone(),
        Some("my-bucket".into()),
    );

    let message_service = MessageService::new(redis_client.clone());
    let history_service = HistoryService::new(s3_client.clone());
    let trigger_service = TriggerService::new("http://localhost:8080/api/v1/export?room=general".into());

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);
        App::new()
            .app_data(web::Data::new(message_service.clone()))
            .app_data(web::Data::new(export_service.clone()))
            .app_data(web::Data::new(history_service.clone()))
            .wrap(cors)
            .wrap(
                DefaultHeaders::new()
                    .add(("X-Content-Type-Options", "nosniff"))
                    .add(("X-Frame-Options", "DENY"))
                    .add(("X-XSS-Protection", "1; mode=block"))
                    .add((
                        "Strict-Transport-Security",
                        "max-age=31536000; includeSubDomains",
                    ))
                    .add(("Referrer-Policy", "no-referrer"))
                    .add(("Permissions-Policy", "geolocation=(), microphone=()")),
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
