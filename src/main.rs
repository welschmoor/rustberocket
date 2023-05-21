#[macro_use]
extern crate rocket;

use rocket::serde::json::{json, Value};

#[get("/users")]
fn get_users() -> Value {
    json!([
      { "id": 1, "name": "Have One"},
      { "id": 2, "name": "Have Two"}
    ])
}

// #[get("/user/<id>")]
// fn get_user_by_id(id: i32) -> Value {
//     json!({ "id": id, "name": "Have One"})
// }

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", routes![get_users])
        .launch()
        .await;
}
