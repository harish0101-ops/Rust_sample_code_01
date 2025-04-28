use crate::db::DbConn;
use crate::models::{NewUser, User};
use crate::schema::users;
use diesel::prelude::*;
use rocket::post;
use rocket::serde::json::Json;

#[post("/users", data = "<user>")]
pub async fn create_user(conn: DbConn, user: Json<NewUser>) -> Json<User> {
    conn.run(move |c| {
        diesel::insert_into(users::table)
            .values(&*user)
            .get_result::<User>(c)
    })
    .await
    .map(Json)
    .unwrap() // per instructions: skip error handling
}
