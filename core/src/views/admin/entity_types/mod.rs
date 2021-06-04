use actix_web::error::InternalError;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, get, Responder, web};
use sailfish::TemplateOnce;
use chrono::{DateTime, Utc};
use crate::models::EntityType;

#[derive(TemplateOnce)]
#[template(path = "admin/entity_types/template.stpl")]
struct Template {
	entity_types: Vec<EntityType>
}

#[get("/entity_types")]
async fn get(data: web::Data<JunkDrawer>) -> impl Responder {
	let entity_types = data.junk.entity_types.unwrap();
	
	let html = Template { entity_types }
		.render_once()
		.map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))
		.unwrap();

	HttpResponse::Ok().content_type("text/html; charset=utf-8").body(html)
}

#[post("/entity_type")]
async fn create(data: web::Data<JunkDrawer>) -> impl Responder {
	let mut entity_types = data.junk.entity_types.lock().unwrap();
	let handle = "testHandle";
	let label = "Test Label";
	let date_created = Utc::now();
	let date_updated = Utc::now();
	
	
	let entity_type_new = EntityType {
		handle,
		label,
		date_created,
		date_updated,
		archived: Default::default()
	}
	
	entity_types.push(entity_type);
	
	let html = Template { entity_types }
		.render_once()
		.map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))
		.unwrap();

	HttpResponse::Ok().content_type("text/html; charset=utf-8").body(html)
}
/*
	#[post("/entity_type/delete")]
	async fn delete(data: web::Data<JunkDrawer>) -> impl Responder {
		
	}
*/

pub fn config(cfg: &mut web::ServiceConfig){
	cfg.service(get);
}

pub fn test() -> String {
	let ctx = Template;
	ctx.render_once().unwrap()
}