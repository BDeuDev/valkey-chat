use actix_web::web;

use crate::controllers::messages::{create_message, get_messages};

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(create_message);
    cfg.service(get_messages);
}
