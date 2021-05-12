// This file includes all of our rocket-related stuff - the server, endpoints, routes, etc.

use anyhow::Result;
use rocket::response::content;

#[get("/", format = "text/html")]
fn index() -> content::Html<&'static str> {
    content::Html(
        "Welcome to my blog!\
    <br/><br/>
    !posts

    Thanks for coming!
    ",
    )
}

/// `run` runs the Rocket.rs server.
pub(crate) fn run() -> Result<()> {
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/", rocket_contrib::serve::StaticFiles::from("./serve"))
        .launch();

    Ok(())
}
