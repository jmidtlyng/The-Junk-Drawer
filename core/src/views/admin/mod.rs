use actix_web::error::InternalError;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, get, Responder, web};
use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "admin/template.stpl")]
struct AdminTemplate {
	messages: Vec<String>,
}

#[get("")]
async fn get() -> impl Responder {
	let messages = vec![String::from("foo"), String::from("bar")];

	let html = AdminTemplate { messages }
		.render_once()
		.map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))
		.unwrap();

	HttpResponse::Ok().content_type("text/html; charset=utf-8").body(html)
}

pub mod entities;

pub fn config(cfg: &mut web::ServiceConfig){
	let entities_config = web::scope("/entities").configure(entities::config);
	
	cfg.service(get)
		.service(entities_config);
}