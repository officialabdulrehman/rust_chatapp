#[macro_use]
extern crate rocket;

use rocket::form::Form;
use rocket::fs::{relative, FileServer};
use rocket::tokio::select;
use rocket::{
    response::stream::{Event, EventStream},
    serde::{Deserialize, Serialize},
    tokio::sync::broadcast::{channel, error::RecvError, Sender},
    Shutdown, State,
};

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

/*
  Receive a message from a form submission and broadcast it to any receivers.
*/
#[post("/message", data = "<form>")]
fn post(form: Form<Message>, queue: &State<Sender<Message>>) {
    // Sending will fail if there are no active listeners

    let _res = queue.send(form.into_inner());
}

/*
  Returns an infinite stream of server-sent events. Each event is a message
  pulled from a broadcast queue sent by the `post` handler.
*/
#[get("/events")]
async fn events(queue: &State<Sender<Message>>, mut end: Shutdown) -> EventStream![] {
    let mut rx = queue.subscribe();

    EventStream! {
      loop {
          let msg = select! {
            msg = rx.recv() => match msg {
              Ok(msg) => msg,
              Err(RecvError::Closed) => break,
              Err(RecvError::Lagged(_)) => continue,
            },
            _ = &mut end => break,
          };

          yield Event::json(&msg)
      }
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(channel::<Message>(1024).0)
        .mount("/", routes![test, post, events])
        .mount("/", FileServer::from(relative!("static")))
}
