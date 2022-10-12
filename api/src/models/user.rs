use crate::schema::users;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Clone, Serialize, Deserialize, Eq, PartialEq, Debug)]
pub struct User {
	pub id: i32,
	pub username: String,
	pub password: String,
	pub created_on: chrono::NaiveDateTime,
}

#[derive(Serialize, Deserialize)]
pub struct UserJSONRequest {
	pub username: String,
	pub password: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUserDB {
	pub username: String,
	pub password: String,
	pub created_on: chrono::NaiveDateTime,
}
