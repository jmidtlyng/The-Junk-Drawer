use actix_web::error::InternalError;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, get, Responder, web};
use sailfish::TemplateOnce;
use chrono::{DateTime, Utc};
use crate::server::JunkDrawer;

struct EntityType {
	handle: String,
	label: String,
	date_created: DateTime<Utc>,
	date_updated DateTime<Utc>,
	archived: bool
}

#[derive(TemplateOnce)]
#[template(path = "admin/entity_types/template.stpl")]
struct Template {
	entity_type: EntityType
}

#[get("/entities")]
async fn get(data: web::Data<JunkDrawer>) -> impl Responder {
	let mut entities = data.junk.lock().unwrap(); // <- get counter's MutexGuard
	*entities += 1;
	//let messages = vec![String::from("foo"), String::from("bar")];

	let html = Template { entities: *entities }
		.render_once()
		.map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))
		.unwrap();

	HttpResponse::Ok().content_type("text/html; charset=utf-8").body(html)
}

pub fn config(cfg: &mut web::ServiceConfig){
	cfg.service(get);
}

pub fn test() -> String {
	let ctx = Template { entities: 1 };
	ctx.render_once().unwrap()
}