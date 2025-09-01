use actix_web::{post, get, web, HttpResponse, Responder};
use crate::controllers::types::MessagePayload;
use crate::services::message::MessageService;

#[post("/message")]
async fn create_message(
    payload: web::Json<MessagePayload>,
    service: web::Data<MessageService>,
) -> impl Responder {
    match service.save_message(payload.into_inner()).await {
        Ok(_) => HttpResponse::Ok().body("Message saved"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/messages/{room}")]
async fn get_messages(
    path: web::Path<String>,
    service: web::Data<MessageService>,
) -> impl Responder {
    let room = path.into_inner();
    match service.get_recent_messages(&room).await {
        Ok(messages) => HttpResponse::Ok().json(messages),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
