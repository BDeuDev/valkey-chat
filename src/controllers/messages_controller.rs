use actix_web::{post, get, web, HttpResponse, Responder};
use crate::controllers::types::{MessagePayload, MessageQuery};
use crate::services::message_service::MessageService;

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

#[get("/messages")]
async fn get_messages(
    query: web::Query<MessageQuery>,
    service: web::Data<MessageService>,
) -> impl Responder {
    let room = &query.room;
    match service.get_recent_messages(&room).await {
        Ok(messages) => HttpResponse::Ok().json(messages),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
