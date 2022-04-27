use super::db::Conn as DbConn;
use rocket_contrib::json::Json;
use super::models::{User, NewUser};
use serde_json::Value;
use crate::models::{UserData, UserLoginData};

#[post("/users", format = "application/json")]
pub fn get_all(conn: DbConn) -> Json<Value> {
    let users = User::get_all_users(&conn);
    Json(json!({
        "status": 200,
        "result": users,
    }))
}

#[post("/newUser", format = "application/json", data = "<new_user>")]
pub fn new_user(conn: DbConn, new_user: Json<NewUser>) -> Json<Value> {

    Json(json!({
        "status":  User::insert_user(new_user.into_inner(), &conn),
    }))
}

#[post("/getUser", format = "application/json", data = "<user_login_data>")]
pub fn find_user(conn: DbConn, user_login_data: Json<UserLoginData>) -> Json<Value> {
    Json(json!({
        "status": 200,
        "result": User::get_user_by_login_password(user_login_data.into_inner(), &conn),
    }))
}

#[post("/updateUser", format = "application/json", data = "<user>")]
pub fn update_user(conn: DbConn, user: Json<UserData>) -> Json<Value> {
    let status = User::update_user(user.into_inner(), &conn);
    Json(json!({
        "status": status
    }))
}