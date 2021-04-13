
use actix_web::error::InternalError;
use actix_web::http::StatusCode;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "admin/index.stpl")]
struct AdminTemplate {
	messages: Vec<String>,
}

async fn admin_template(req: HttpRequest) -> actix_web::Result<HttpResponse> {
	let messages = vec![String::from("foo"), String::from("bar")];
		
	let body = AdminTemplate { messages }
		.render_once()
		.map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;

	Ok(HttpResponse::Ok()
		.content_type("text/html; charset=utf-8")
		.body(body))
}