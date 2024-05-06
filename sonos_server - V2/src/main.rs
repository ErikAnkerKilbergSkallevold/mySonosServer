mod error_routes;
mod lofty;
mod routes;
mod sonos;
mod utils;

#[macro_use]
extern crate rocket;

use rocket::fs::FileServer;

use crate::error_routes::{internal_error, not_found};
use crate::routes::{get_sound_uri, index, play_sound_in_rooms};

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount(
            "/api",
            routes![play_sound_in_rooms, get_sound_uri],
        )
        .mount("/public", FileServer::from("./static/"))
        .register("/", catchers![not_found, internal_error])
}
