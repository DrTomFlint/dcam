#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, this is the Outpost Hub."
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
