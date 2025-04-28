use crate::schema::{posts, users, posts_tags}; 
use diesel::prelude::*;
use diesel::Identifiable;
use serde::{Deserialize, Serialize};
use diesel::sql_types::{Integer, Text, Nullable};
use diesel::deserialize::FromSqlRow;



#[derive(Queryable, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Insertable, Deserialize, Serialize)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Queryable, Debug, Identifiable, Associations, Serialize, Deserialize)]
#[diesel(table_name = posts)]
#[diesel(belongs_to(User, foreign_key = created_by))]
pub struct Post {
    pub id: i32,
    pub created_by: i32,  // must be second
    pub title: String,
    pub body: String,
}

#[derive(Insertable, Deserialize, Serialize)]
#[diesel(table_name = posts)]
pub struct NewPost {
    pub created_by: i32,
    pub title: String,
    pub body: String, 
}

#[derive(Deserialize)]
pub struct PostApiInput {
    pub post: NewPost,    
    pub tags: Option<Vec<String>>, 
}

#[derive( Debug, Serialize, Deserialize)]
#[diesel(table_name = posts_tags)]
pub struct NewPostTag {
    pub post_id: i32,
    pub tag:  String,
}



#[derive(QueryableByName, Debug, Serialize, Deserialize)]
pub struct PostWithUserAndTags {
    #[sql_type = "Integer"]
    pub id: i32,

    #[sql_type = "Text"]
    pub title: String,

    #[sql_type = "Text"]
    pub body: String,

    #[sql_type = "Array<Text>"]
    pub tags: Vec<String>,

    #[sql_type = "Nullable<Integer>"]
    pub user_id: Option<i32>,

    #[sql_type = "Nullable<Text>"]
    pub username: Option<String>,

    #[sql_type = "Nullable<Text>"]
    pub first_name: Option<String>,

    #[sql_type = "Nullable<Text>"]
    pub last_name: Option<String>,
}



