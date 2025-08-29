use actix_cors::Cors;
use actix_web::{middleware::DefaultHeaders, web, App, HttpServer};
use redis::Client;
use tokio;

use crate::routes::init_routes;
mod storage;
mod routes;
mod services;
#[derive(Clone)]
pub struct AppState {
    redis_client: Client,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "redis://127.0.0.1:6379";
    let client = Client::open(url)?;
    let state = AppState {
        redis_client: client,
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
