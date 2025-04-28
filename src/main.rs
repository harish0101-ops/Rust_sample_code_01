#[macro_use]
extern crate rocket;

mod db;
mod models;
mod routes;
mod schema;

use db::DbConn;

#[launch]
fn rocket() -> _ {
    rocket::build().attach(DbConn::fairing()).mount(
        "/",
        routes![
            routes::users::create_user,
            routes::posts::create_post,
            routes::posts::list_posts,
        ],
    )
}
