use super::db::Conn as DbConn;
use rocket_contrib::json::Json;
use super::models::{User, NewUser};
use serde_json::Value;
use crate::models::UserData;
// this is to get users from the database
#[post("/get_all", format = "application/json")] //here
pub fn get_all(conn: DbConn) -> Json<Value> {
    let users = User::get_all_users(&conn);
    Json(json!({
        "status": 200,
        "result": users,
    }))
}
#[post("/new_token", format = "application/json", data = "<new_user>")]
pub fn new_user(conn: DbConn, new_user: Json<NewUser>) -> Json<Value> {
    Json(json!({
        "status": User::insert_user(new_user.into_inner(), &conn),
        "result": User::get_all_users(&conn).first(),
    }))
}
#[post("/get_url", format = "application/json", data = "<user_data>")]
pub fn find_user(conn: DbConn, user_data: Json<UserData>) -> Json<Value> {
    Json(json!({
        "status": 200,
        "result": User::get_user_by_username(user_data.into_inner(), &conn),
    }))
}