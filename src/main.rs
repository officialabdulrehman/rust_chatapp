#[macro_use]
extern crate rocket;

use rocket::{
    serde::{Deserialize, Serialize},
    tokio::sync::broadcast::channel,
    tokio::sync::broadcast::Sender,
    State,
};

use rocket::form::Form;

#[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]

struct Message {
    #[field(validate = len(..30))]
    pub room: String,
    #[field(validate = len(..20))]
    pub username: String,
    pub message: String,
}

#[get("/hello-world")]
fn test() -> &'static str {
    "Meow World!"
}

#[post("/message", data = "<form>")]
fn post(form: Form<Message>, queue: &State<Sender<Message>>) {
    // Sending will fail if there are no active listeners

    let _res = queue.send(form.into_inner());
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(channel::<Message>(1024).0)
        .mount("/test", routes![test])
}
