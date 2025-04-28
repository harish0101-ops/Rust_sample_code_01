// src/db.rs

use rocket_sync_db_pools::{database, diesel};

#[database("blog_task_1")]
pub struct DbConn(diesel::PgConnection);
