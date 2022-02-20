use super::db::Conn as DbConn;
use rocket_contrib::json::Json;
use super::models::{
    User, NewUserData,
    Post, NewPostData, PostIdData,
};
use serde_json::Value;
use crate::models::UserNameData;

#[get("/users")]
pub fn get_all_users(conn: DbConn) -> Json<Value> {
    let users = User::get_all(&conn);
    Json(json!({
        "status": 200,
        "result": users,
    }))
}

#[post("/new-user", format = "application/json", data = "<new_user>")]
pub fn new_user(conn: DbConn, new_user: Json<NewUserData>) -> Json<Value> {
    Json(json!({
        "status": User::insert(new_user.into_inner(), &conn),
        "result": User::get_all(&conn).first(),
    }))
}

#[post("/get-user", format = "application/json", data = "<user_data>")]
pub fn find_user(conn: DbConn, user_data: Json<UserNameData>) -> Json<Value> {
    let result = User::get_by_username(user_data.into_inner(), &conn);

    Json(json!({
        "status": if result.len() == 0 { 404 } else { 200 },
        "result": result,
    }))
}

#[post("/remove-user", format = "application/json", data = "<user_data>")]
pub fn remove_user(conn: DbConn, user_data: Json<UserNameData>) -> Json<Value> {
    User::remove_by_username(user_data.into_inner(), &conn);

    Json(json!({
        "status": 200
    }))
}

#[get("/posts")]
pub fn get_all_posts(conn: DbConn) -> Json<Value> {
    let posts = Post::get_all(&conn);
    Json(json!({
        "status": 200,
        "result": posts,
    }))
}

#[post("/new-post", format = "application/json", data = "<new_post>")]
pub fn new_post(conn: DbConn, new_post: Json<NewPostData>) -> Json<Value> {
    Json(json!({
        "status": Post::insert(new_post.into_inner(), &conn),
        "result": Post::get_all(&conn).first(),
    }))
}

#[post("/get-post", format = "application/json", data = "<post_data>")]
pub fn find_post(conn: DbConn, post_data: Json<PostIdData>) -> Json<Value> {
    let result = Post::get(post_data.into_inner(), &conn);

    Json(json!({
        "status": if result.len() == 0 { 404 } else { 200 },
        "result": result,
    }))
}

#[post("/remove-post", format = "application/json", data = "<post_data>")]
pub fn remove_post(conn: DbConn, post_data: Json<PostIdData>) -> Json<Value> {
    Post::remove(post_data.into_inner(), &conn);

    Json(json!({
        "status": 200
    }))
}
