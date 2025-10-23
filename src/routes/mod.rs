pub mod home;
pub mod user;

use actix_web::web;

use crate::middleware::middleware::Jwt;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(home::home);
    cfg.service(user::register);
    cfg.service(user::login);
    cfg.service(web::scope("/user").wrap(Jwt).service(user::verify));
}
