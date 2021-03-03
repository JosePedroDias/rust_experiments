// https://rocket.rs/master/guide/
// https://rocket.rs/master/guide/requests/#default-ranking
// https://rocket.rs/master/guide/requests/#forwarding-guards
// https://api.rocket.rs/v0.4/rocket_contrib/

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;

use rocket::data::{Data, ToByteUnit};
use rocket::http::RawStr;
use rocket::response::Debug;
use rocket::tokio::time::{sleep, Duration};
use rocket::Request;
use rocket_contrib::json::Json;
use rocket_contrib::serve::StaticFiles;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    age: usize,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/hello/<name>")]
fn hello(name: &RawStr) -> String {
    format!("Hello, {}!", name)
}

#[get("/page/<path..>")]
fn get_page(path: PathBuf) -> String {
    format!("Hello, {:?}!", path)
}

#[get("/hi?<wave>&<name>")]
fn hi(name: Option<String>, wave: Option<String>) -> String {
    format!("{:?} {:?}", name, wave)
}

#[post("/user", format = "application/json", data = "<user>")]
fn new_user(user: Json<User>) -> String {
    if user.age > 42 {
        return String::from("too old");
    } else if user.age < 18 {
        return String::from("too young");
    }
    format!("{:?}", user)
}

#[get("/user")]
fn get_user() -> Json<User> {
    return Json(User {
        name: String::from("Jesus"),
        age: 33,
    });
}

#[post("/upload", format = "plain", data = "<data>")]
async fn upload(data: Data) -> Result<String, Debug<std::io::Error>> {
    let bytes_written = data
        .open(128.kibibytes())
        .stream_to_file("/tmp/upload.txt")
        .await?;

    Ok(bytes_written.to_string())
}

#[get("/delay/<seconds>")]
async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("Waited for {} seconds", seconds)
}

#[catch(404)]
fn not_found(_req: &Request) -> String {
    String::from("four oh four")
}

#[launch]
fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount(
            "/",
            routes![index, hello, get_page, hi, upload, delay, new_user, get_user],
        )
        .mount("/public", StaticFiles::from("public"))
        .register(catchers![not_found])
}
