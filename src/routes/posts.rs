use crate::db::DbConn;
use crate::models::{PostApiInput, Post, User, PostWithUserAndTags};
use crate::schema::posts;
use crate::schema::posts::dsl::*;
use crate::schema::users::dsl::{id as user_id, users as users_table};
use diesel::prelude::*;
use crate::models::NewPostTag;
use rocket::serde::Serialize;
use diesel::dsl::*;
use diesel::sql_types::Text;
use rocket::{get, post, serde::json::Json};
use crate::models::PostWithTags;

#[post("/posts", data = "<new_post>")]
pub async fn create_post(
    conn: DbConn,
    new_post: Json<PostApiInput>, // This now includes tags
) -> Json<Post> {
    use crate::schema::posts;
    use crate::schema::posts_tags;

    conn.run(move |c| {
        // Insert the post into the posts table
        let inserted_post: Post = diesel::insert_into(posts::table)
            .values(&new_post.post)
            .get_result(c)
            .unwrap();

        // Insert tags into the posts_tags table if they exist
        if let Some(tags) = &new_post.tags {
            for tag in tags {
                diesel::insert_into(posts_tags::table)
                    .values(NewPostTag {
                        post_id: inserted_post.id,
                        tag: tag.clone(),
                    })
                    .execute(c)
                    .unwrap();
            }
        }

        Json(inserted_post)
    })
    .await
}



#[derive(Serialize)]
pub struct PaginatedPost {
    pub post: Post,
    pub tags: Vec<String>,
}

#[derive(Serialize)]
pub struct PaginatedPosts {
    records: Vec<(PaginatedPost, User)>,
    meta: PaginationMeta,
}

#[derive(Serialize)]
pub struct PaginationMeta {
    current_page: i64,
    per_page: i64,
    from: i64,
    to: i64,
    total_pages: i64,
    total_docs: i64,
}

#[get("/posts?<page>&<limit>&<search>")]
pub async fn list_posts(
    conn: DbConn,
    page: Option<i64>,
    limit: Option<i64>,
    search: Option<String>,
) -> Json<PaginatedPosts> {
    use diesel::prelude::*;
    use diesel::dsl::*;
    use diesel::sql_types::{Integer, Text, Nullable, Array};
    use crate::schema::{posts, posts_tags, users};

    let page = page.unwrap_or(1);
    let limit = limit.unwrap_or(10);
    let offset = (page - 1) * limit;

    conn.run(move |c| {
        let mut query = posts::table
            .left_join(posts_tags::table.on(posts_tags::post_id.eq(posts::id)))
            .left_join(users::table.on(users::id.nullable().eq(posts::created_by)))
            .select((
                posts::id,
                posts::title,
                posts::body,
                sql::<Array<Text>>("ARRAY_AGG(posts_tags.tag)"),
                users::id.nullable(),
                users::username.nullable(),
                users::first_name.nullable(),
                users::last_name.nullable(),
            ))
            .group_by((posts::id, users::id, users::username, users::first_name, users::last_name))
            .order_by(posts::id)
            .offset(offset)
            .limit(limit)
            .into_boxed();

        if let Some(s) = &search {
            query = query.filter(posts::title.ilike(format!("%{}%", s)));
        }

        let raw_results: Vec<PostWithUserAndTags> = query
            .load(c)
            .expect("Failed to load posts with user and tags");

        let total_docs: i64 = posts::table
            .filter(posts::title.ilike(format!("%{}%", search.clone().unwrap_or_default())))
            .count()
            .get_result(c)
            .unwrap_or(0);

        let results = raw_results
            .into_iter()
            .map(|r| PaginatedPost {
                post: Post {
                    id: r.id,
                    title: r.title,
                    body: r.body,
                    created_by: match r.user_id {
                        Some(user_id) => Some(UserInfo {
                            user_id,
                            username: r.username.unwrap_or_default(),
                            first_name: r.first_name,
                            last_name: r.last_name,
                        }),
                        None => None,
                    },
                },
                tags: r.tags,
            })
            .collect();

        Json(PaginatedPosts {
            records: results,
            meta: PaginationMeta {
                current_page: page,
                per_page: limit,
                from: offset + 1,
                to: (offset + results.len() as i64).min(total_docs),
                total_pages: (total_docs as f64 / limit as f64).ceil() as i64,
                total_docs,
            },
        })
    })
    .await
}
