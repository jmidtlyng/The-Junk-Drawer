use actix_web::error::InternalError;
use actix_web::http::StatusCode;
use actix_web::{get, HttpResponse, Responder};
use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "admin/entities/index.stpl")]
struct AdminTemplate {
	messages: Vec<String>,
}

#[get("/entities")]
async fn get() -> impl Responder {
	let messages = vec![String::from("foo"), String::from("bar")];

	let html = AdminTemplate { messages }
		.render_once()
		.map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))
		.unwrap();

	HttpResponse::Ok()
		.content_type("text/html; charset=utf-8")
		.body(html)
}
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