use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;

use crate::{ AppState};
use crate::storage::{messages::save};
#[derive(Deserialize)]
pub struct MessagePayload {
    pub room: String,
    pub user: String,
    pub text: String,
}

#[post("/message")]
async fn save_message(
    state: web::Data<AppState>,
    payload: web::Json<MessagePayload>,
) -> impl Responder {
    let mut conn = match state.redis_client.get_multiplexed_async_connection().await {
        Ok(c) => c,
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
    };

    if let Err(e) = save(
        &mut conn,
        &payload.room,
        &payload.user,
        &payload.text,
    ).await {
        return HttpResponse::InternalServerError().body(e.to_string());
    }

    HttpResponse::Ok().body("Message saved")
}