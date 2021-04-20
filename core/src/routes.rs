use actix_web::web;
mod views;
/*
use views;
#[macro_export]
macro_rules! 
#[get("/")]
async fn index() -> impl Responder {
    let html = views::templates::index::html();
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

#[get("/map")]
async fn map() -> impl Responder {
    let html = views::templates::map::html();
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

#[get("/details")]
async fn details() -> impl Responder {
    let html = views::templates::details::html();
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}
*/

pub fn config(cfg: &mut web::ServiceConfig) {
    let admin_scope = web::scope("/admin").service(views::admin_template);
    
    cfg.service( admin_scope );
}
