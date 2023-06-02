use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

use self::models::{NewPost, Post};

pub mod models;
pub mod schema;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn main() {
    let connection = &mut establish_connection();

    // create post
    // let title = String::from("TypeScript Programming");
    // let body = String::from("TypeScript description.");
    //
    // let new_post = NewPost {
    //     title: &title,
    //     body: &body,
    //     published: false,
    // };
    //
    // let post = create_post(connection, new_post);
    //
    // println!("Created post: {}, with id: {}", post.title, post.id);

    // get first five posts unpublished
    // let mut results = get_first_five_posts_published(connection);
    // for post in results.iter_mut() {
    //     post.published = false;
    //
    //     update_post(connection, post);
    // }
    // let pub_results = get_first_five_posts_not_published(connection);

    // deleting post
    // let rows_deleted = delete_post(connection, 7);
    // println!("Deleted {} rows", rows_deleted);

    // get all posts
    let all_results = get_all_posts(connection);

    println!("Displaying {} posts", all_results.len());

    for post in all_results {
        println!("-----------");
        println!("Id: {}", post.id);
        println!("Title: {}", post.title);
        println!("-----------\n");
    }
}

pub fn delete_post(connection: &mut MysqlConnection, post_id: i32) -> usize {
    use self::schema::posts::dsl::*;

    let num_deleted = diesel::delete(posts.filter(id.eq(post_id)))
        .execute(connection)
        .expect("Error deleting posts");

    num_deleted
}

pub fn create_post(conn: &mut MysqlConnection, new_post: NewPost) -> Post {
    use crate::schema::posts;

    conn.transaction(|conn| {
        diesel::insert_into(posts::table)
            .values(&new_post)
            .execute(conn)?;

        posts::table
            .order(posts::id.desc())
            .select(Post::as_select())
            .first(conn)
    })
    .expect("Error while saving post")
}

fn get_all_posts(connection: &mut MysqlConnection) -> Vec<Post> {
    use self::schema::posts::dsl::*;

    let results = posts
        .select(Post::as_select())
        .load(connection)
        .expect("Error loading posts");

    results
}

fn get_first_five_posts_published(connection: &mut MysqlConnection) -> Vec<Post> {
    use self::schema::posts::dsl::*;

    let results = posts
        .filter(published.eq(true))
        .limit(5)
        .select(Post::as_select())
        .load(connection)
        .expect("Error loading posts");

    results
}

fn get_first_five_posts_not_published(connection: &mut MysqlConnection) -> Vec<Post> {
    use self::schema::posts::dsl::*;

    let results = posts
        .filter(published.eq(false))
        .limit(5)
        .select(Post::as_select())
        .load(connection)
        .expect("Error loading posts");

    results
}

fn update_post(connection: &mut MysqlConnection, post_to_update: &Post) -> Post {
    use self::schema::posts::dsl::*;

    let post = connection
        .transaction(|connection| {
            let post = posts
                .find(post_to_update.id)
                .select(Post::as_select())
                .first(connection)?;

            diesel::update(posts.find(post_to_update.id))
                .set(post_to_update)
                .execute(connection)?;

            Ok(post)
        })
        .unwrap_or_else(|_: diesel::result::Error| {
            panic!("Error while updating post {}", post_to_update.id)
        });

    post
}
