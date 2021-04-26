use actix_web::web;
use crate::views;

pub fn config(cfg: &mut web::ServiceConfig) {
    let admin_scope = web::scope("/admin")
        .service(views::admin::index::get)
        .service(views::admin::entities::index::get);
    
    cfg.service( admin_scope );
}
