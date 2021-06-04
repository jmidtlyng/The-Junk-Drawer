use chrono::{DateTime, Utc};

pub struct FieldType {
	handle: String,
	label: String,
	date_created: DateTime<Utc>,
	date_updated: DateTime<Utc>,
	archived: bool
}

pub struct Field {
	handle: String,
	label: String,
	r#type: FieldType,
	date_created: DateTime<Utc>,
	date_updated: DateTime<Utc>,
	archived: bool
}

pub struct Entity {
	handle: String,
	label: String,
	r#type: EntityType,
	date_created: DateTime<Utc>,
	date_updated: DateTime<Utc>,
	archived: bool
}

pub struct EntityType {
	handle: String,
	label: String,
	date_created: DateTime<Utc>,
	date_updated: DateTime<Utc>,
	archived: bool
}

pub struct User {
	username: String,
	first_name: String,
	last_name: String,
	password: String,
	emails: Vec<UserEmail>,
	groups: Vec<UserGroup>,
	date_created: DateTime<Utc>,
	date_updated: DateTime<Utc>,
	archived: bool
}

pub struct UserEmail {
	address: String,
	opt_out: bool
}

pub struct UserGroup {
	handle: String,
	label: String,
	last_name: String,
	password: String,
	date_created: DateTime<Utc>,
	date_updated: DateTime<Utc>,
	archived: bool
}

pub struct Junk {
	entity_types: Vec<EntityType>,
	entities: Vec<Entity>,
	fields: Vec<Field>,
	users: Vec<User>,
	user_groups: Vec<UserGroup>
}

pub struct JunkDrawer {
	pub junk: Vec<Junk>
}