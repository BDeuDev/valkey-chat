use actix_web::web;

use crate::services::{export_messages::export_messages, read_history::{read_history}, save_message::save_message};

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(save_message);
    cfg.service(export_messages);
    cfg.service(read_history);
}
