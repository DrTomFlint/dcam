#[macro_use] extern crate rocket;
#[macro_use] extern crate libcamera;

#[get("/")]
fn index() -> &'static str {
    "Hello, this is an Outpost Camera on PiZeroW2 at cam4.local."
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
