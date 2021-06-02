use chrono::{DateTime, Utc};

pub enum FieldType {
	Assets,
	Categories,
	Color,
	DateTime,
	Dropdown,
	Entities,
	Lightswitch
	Matrix
	MultiSelect,
	Number,
	PlainText,
	PositionSelect,
	RadioButtons,
	RichText,
	Table,
	Tags,
	Users
}

pub struct Field {
	handle: String,
	label: String,
	type: FieldType,
	date_created: DateTime<Utc>,
	date_updated DateTime<Utc>,
	archived: bool
}

pub struct Entity {
	handle: String,
	label: String,
	type: EntityType,
	date_created: DateTime<Utc>,
	date_updated DateTime<Utc>,
	archived: bool
}

pub struct EntityType {
	handle: String,
	label: String,
	date_created: DateTime<Utc>,
	date_updated DateTime<Utc>,
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
	date_updated DateTime<Utc>,
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
	date_updated DateTime<Utc>,
	archived: bool
}

pub struct Junk {
	entity_types: Vec<EntityType>,
	entities: Vec<Entities>,
	fields: Vec<Field>,
	users: Vec<User>,
	user_groups: Vec<UserGroup>
}

pub struct JunkDrawer {
	pub junk: Vec<Junk>
}