use rand::seq::SliceRandom;
use rocket::{
    serde::Serialize,
    response::content::RawHtml,
    fs::NamedFile
};
use std::{
    collections::BTreeMap,
    path::{PathBuf, Path}
};
use handlebars::Handlebars;

mod utils;

use utils::toml::*;

#[macro_use]
extern crate rocket;

#[derive(Debug, Serialize)]
struct NoteDetails {
    title: String,
    content: String,
}

#[get("/")]
fn index() -> RawHtml<String> {
    let mut handlebars = Handlebars::new();
    handlebars.register_template_file("index", "templates/index.hbs").unwrap();

    let posts = get_posts().posts.into_iter().map(|(_uuid, content)| {
        NoteDetails {
            title: content.title,
            content: content.content,
        }
    }).collect::<Vec<NoteDetails>>();

    let mut data = BTreeMap::new();
    data.insert("posts".to_string(), posts);

    let handlebars_output = handlebars.render("index", &data).unwrap();

    //render as html with css
    RawHtml(handlebars_output)
}

#[get("/add_note")]
fn add_note() -> RawHtml<String> {
    let mut handlebars = Handlebars::new();
    handlebars.register_template_file("add_note", "templates/add_note.hbs").unwrap();

    let handlebars_output = handlebars.render("add_note", &()).unwrap();

    //render as html with css
    RawHtml(handlebars_output)
}

#[get("/static/<file..>")]
async fn static_files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).await.ok()
}

#[catch(404)]
fn not_found() -> rocket::response::status::NotFound<Option<RawHtml<String>>> {
    let mut handlebars = Handlebars::new();
    handlebars.register_template_file("404", "templates/error.hbs").unwrap();

    let messages = vec![
        "Oops! Looks like this page got stuck in the wrong place.".to_string(),
        "Sorry, we're a bit sticky-fingered and misplaced that page.".to_string(),
        "This page has gone rogue and escaped. We're working on finding it.".to_string(),
        "Uh oh, looks like this page fell off the grid.".to_string(),
        "This page has gone missing. We're working on finding it.".to_string(),
        "Looks like this note got lost in translation. We'll get it sorted ASAP.".to_string(),
        "This page seems to have gone on a solo adventure. We'll track it down soon!".to_string(),
        "We apologize for this sticky situation. We're working on getting things back to normal.".to_string(),
        "Whoops! This page took a wrong turn somewhere. We'll redirect it shortly.".to_string(),
        "Sorry for the mess! This page got a little too attached to its own ideas.".to_string()
    ];

    let message = messages.choose(&mut rand::thread_rng()).unwrap();
    
    BTreeMap::new().insert("error_message".to_string(), message.to_string());
    let mut data = BTreeMap::new();
    data.insert("error_message".to_string(), message.to_string());

    let handlebars_output = handlebars.render("404", &data).unwrap();

    rocket::response::status::NotFound(Some(RawHtml(handlebars_output)))
}

#[catch(500)]
fn internal_error() -> rocket::response::status::NotFound<Option<RawHtml<String>>> {
    let mut handlebars = Handlebars::new();
    handlebars.register_template_file("500", "templates/error.hbs").unwrap();

    let messages = vec![
        "Looks like the notes got stuck together. We're carefully peeling them apart.".to_string(),
        "We accidentally knocked over our sticky note tower. We're rebuilding it now.".to_string(),
        "We accidentally knocked over our sticky note tower. We're rebuilding it now.".to_string(),
        "Our notes are in a bit of a jam. We're trying to unstick them.".to_string(),
        "The sticky notes got mixed up. We're sorting them out now.".to_string(),
        "Looks like we ran out of sticky notes. We're restocking ASAP.".to_string(),
        "The sticky notes are having a party and forgot to invite the server. We're crashing the party now.".to_string(),
        "The sticky notes got a little too sticky and caused a server malfunction. We're cleaning up the mess.".to_string(),
        "Our notes got lost in a sea of yellow. We're trying to find the right one.".to_string(),
        "The sticky notes are rebelling against the server. We're negotiating a truce.".to_string()
    ];

    let message = messages.choose(&mut rand::thread_rng()).unwrap();

    BTreeMap::new().insert("error_message".to_string(), message.to_string());
    let mut data = BTreeMap::new();
    data.insert("error_message".to_string(), message.to_string());

    let handlebars_output = handlebars.render("500", &data).unwrap();

    rocket::response::status::NotFound(Some(RawHtml(handlebars_output)))
}

#[shuttle_service::main]
async fn rocket() -> shuttle_service::ShuttleRocket {
    let rocket = rocket::build().mount("/", routes![index, add_note, static_files]).register("/", catchers![not_found, internal_error]);

    Ok(rocket)
}