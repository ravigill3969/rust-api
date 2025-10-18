pub mod home;
pub mod user;

use actix_web::web;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(home::home);
    cfg.service(user::register);
}
