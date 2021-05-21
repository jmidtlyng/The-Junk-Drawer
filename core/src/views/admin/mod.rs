use actix_web::error::InternalError;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, get, Responder, web};
use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "admin/template.stpl")]
struct Template {
	//messages: Vec<String>,
}

#[get("")]
async fn get() -> impl Responder {
	//let messages = vec![String::from("foo"), String::from("bar")];
	// let html = AdminTemplate { messages }
	let html = Template {}
		.render_once()
		.map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))
		.unwrap();

	HttpResponse::Ok().content_type("text/html; charset=utf-8").body(html)
}

pub mod entities;
pub mod fields;

pub fn config(cfg: &mut web::ServiceConfig){
	//let entities_config = web::scope("/entities").configure(entities::config);
	//let fields_config = web::scope("/fields").configure(fields::config);
	cfg.service(get);
	entities::config(cfg);
	fields::config(cfg);
}

pub fn test() -> String {
	let ctx = Template {};
	ctx.render_once().unwrap()
}