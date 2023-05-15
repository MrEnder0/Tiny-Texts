use rocket::{
    serde::Serialize,
    response::content::RawHtml,
    fs::NamedFile,
    form::Form
};
use std::{
    collections::BTreeMap,
    path::{PathBuf, Path}
};
use rand::seq::SliceRandom;
use handlebars::Handlebars;

mod utils;

use utils::toml;

#[macro_use]
extern crate rocket;

#[derive(Debug, Serialize)]
struct NoteDetails {
    title: String,
    content: String,
}

#[derive(FromForm)]
struct SubmittedNote {
    title: String,
    content: String,
}

#[get("/")]
fn index() -> RawHtml<String> {
    let mut handlebars = Handlebars::new();
    handlebars.register_template_file("index", "static/templates/index.hbs").unwrap();
    handlebars.register_template_file("github_link", "static/templates/github_link.hbs").unwrap();

    let notes = toml::get_notes()
        .notes
        .into_iter()
        .map(|(_uuid, content)| NoteDetails {
            title: content.title,
            content: content.content,
        })
        .collect::<Vec<NoteDetails>>();

    let mut data = BTreeMap::new();
    data.insert("notes".to_string(), notes);

    let handlebars_output = handlebars.render("index", &data).unwrap();

    RawHtml(handlebars_output)
}



#[get("/add_note")]
fn add_note() -> RawHtml<String> {
    let mut handlebars = Handlebars::new();
    handlebars.register_template_file("add_note", "static/templates/add_note.hbs").unwrap();

    let github_link = std::fs::read_to_string("static/templates/github_link.hbs").unwrap();

    let mut data = BTreeMap::new();
    data.insert("github_link".to_string(), github_link.to_string());

    let handlebars_output = handlebars.render("add_note", &data).unwrap();

    //render as html with css
    RawHtml(handlebars_output)
}

#[post("/add_note", data = "<user_input>")]
fn create_note(user_input: Form<SubmittedNote>) -> rocket::response::Redirect {
    let title = user_input.title.clone();
    let content = user_input.content.clone();
    toml::add_note(title, content);

    rocket::response::Redirect::to("/")
}

#[get("/static/<file..>")]
async fn static_files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).await.ok()
}

#[catch(404)]
fn not_found() -> rocket::response::status::NotFound<Option<RawHtml<String>>> {
    let mut handlebars = Handlebars::new();
    handlebars.register_template_file("404", "static/templates/error.hbs").unwrap();

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

    let error_message = messages.choose(&mut rand::thread_rng()).unwrap();
    let github_link = std::fs::read_to_string("static/templates/github_link.hbs").unwrap();

    let mut data = BTreeMap::new();
    data.insert("error_message".to_string(), error_message.to_string());
    data.insert("github_link".to_string(), github_link.to_string());

    let handlebars_output = handlebars.render("404", &data).unwrap();

    rocket::response::status::NotFound(Some(RawHtml(handlebars_output)))
}

#[catch(500)]
fn internal_error() -> rocket::response::status::NotFound<Option<RawHtml<String>>> {
    let mut handlebars = Handlebars::new();
    handlebars.register_template_file("500", "static/templates/error.hbs").unwrap();

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

    let error_message = messages.choose(&mut rand::thread_rng()).unwrap();
    let github_link = std::fs::read_to_string("static/templates/github_link.hbs").unwrap();

    let mut data = BTreeMap::new();
    data.insert("error_message".to_string(), error_message.to_string());
    data.insert("github_link".to_string(), github_link.to_string());

    let handlebars_output = handlebars.render("500", &data).unwrap();

    rocket::response::status::NotFound(Some(RawHtml(handlebars_output)))
}

#[launch]
fn rocket() -> _ {
    if !Path::new("notes.toml").exists() {
        toml::gen_notes();
    }

    rocket::build().mount("/", routes![index, add_note, create_note, static_files]).register("/", catchers![not_found, internal_error])
}