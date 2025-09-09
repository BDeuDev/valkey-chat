use actix_web::web;

use crate::controllers::{export_controller::export_messages_by_room, history_controller::get_history_by_room, messages_controller::{create_message, get_messages}};

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(create_message);
    cfg.service(get_messages);
    cfg.service(export_messages_by_room);
    cfg.service(get_history_by_room);
}
