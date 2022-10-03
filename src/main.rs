#[macro_use]
extern crate rocket;

#[get("/hello-world")]
fn test() -> &'static str {
    "Meow World!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/test", routes![test])
}
