use super::db::Conn as DbConn;
use rocket_contrib::json::Json;
use super::models::{User, NewUser};
use serde_json::Value;
use crate::models::UserData;

#[get("/users")]
pub fn get_all(conn: DbConn) -> Json<Value> {
    let users = User::get_all_users(&conn);
    Json(json!({
        "status": 200,
        "result": users,
    }))
}

#[post("/new-user", format = "application/json", data = "<new_user>")]
pub fn new_user(conn: DbConn, new_user: Json<NewUser>) -> Json<Value> {
    Json(json!({
        "status": User::insert_user(new_user.into_inner(), &conn),
        "result": User::get_all_users(&conn).first(),
    }))
}

#[post("/get-user", format = "application/json", data = "<user_data>")]
pub fn find_user(conn: DbConn, user_data: Json<UserData>) -> Json<Value> {
    let result = User::get_user_by_username(user_data.into_inner(), &conn);

    Json(json!({
        "status": if result.len() == 0 { 404 } else { 200 },
        "result": result,
    }))
}

#[post("/remove-user", format = "application/json", data = "<user_data>")]
pub fn remove_user(conn: DbConn, user_data: Json<UserData>) -> Json<Value> {
    User::remove_user_by_username(user_data.into_inner(), &conn);

    Json(json!({
        "status": 200
    }))
}