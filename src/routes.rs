use super::db::Conn as DbConn;
use rocket_contrib::json::Json;
use rocket::http::RawStr;
use super::models::{
    User, NewUserData,
    Post, NewPostData,
};
use serde_json::Value;

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

#[get("/user/<id>")]
pub fn get_user(conn: DbConn, id: i32) -> Json<Value> {
    let result = User::get(id, &conn);

    Json(json!({
        "status": if result.len() == 0 { 404 } else { 200 },
        "result": result,
    }))
}

#[get("/user/<name>", rank = 2)]
pub fn get_user_by_name(conn: DbConn, name: &RawStr) -> Json<Value> {
    let result = User::get_by_username(name.as_str(), &conn);

    Json(json!({
        "status": if result.len() == 0 { 404 } else { 200 },
        "result": result,
    }))
}

#[get("/remove-user/<id>")]
pub fn remove_user(conn: DbConn, id: i32) -> Json<Value> {
    User::remove(id, &conn);

    Json(json!({
        "status": 200
    }))
}

#[get("/remove-user/<name>", rank = 2)]
pub fn remove_user_by_username(conn: DbConn, name: &RawStr) -> Json<Value> {
    User::remove_by_username(name.as_str(), &conn);

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

#[get("/post/<id>")]
pub fn get_post(conn: DbConn, id: i32) -> Json<Value> {
    let result = Post::get(id, &conn);

    Json(json!({
        "status": if result.len() == 0 { 404 } else { 200 },
        "result": result,
    }))
}

#[get("/remove-post/<id>")]
pub fn remove_post(conn: DbConn, id: i32) -> Json<Value> {
    Post::remove(id, &conn);

    Json(json!({
        "status": 200
    }))
}
