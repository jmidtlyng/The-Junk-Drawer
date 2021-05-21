use actix_web::web;
use crate::views;

pub fn config(cfg: &mut web::ServiceConfig) {
    let admin_scope = web::scope("/admin").configure(views::admin::config);

    cfg.service( admin_scope );
}
