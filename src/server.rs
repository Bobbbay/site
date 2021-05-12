// This file includes all of our rocket-related stuff - the server, endpoints, routes, etc.

use anyhow::Result;
use rocket::response::content;

/// `run` runs the Rocket.rs server.
pub(crate) fn run() -> Result<()> {
    rocket::ignite()
        // TODO: This is a great opportunity to write some more Rust code.
        // .mount("/", routes![index])
        .mount("/", rocket_contrib::serve::StaticFiles::from("./serve"))
        .launch();

    Ok(())
}
