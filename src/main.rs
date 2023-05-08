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
struct BlogPost {
    title: String,
    content: String,
}

#[get("/")]
fn index() -> RawHtml<String> {
    let mut handlebars = Handlebars::new();
    handlebars.register_template_file("index", "templates/index.hbs").unwrap();

    let posts = get_posts().posts.into_iter().map(|(title, content)| {
        BlogPost {
            title,
            content
        }
    }).collect::<Vec<BlogPost>>();

    let mut data = BTreeMap::new();
    data.insert("posts".to_string(), posts);

    let handlebars_output = handlebars.render("index", &data).unwrap();

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
    handlebars.register_template_file("404", "templates/404.hbs").unwrap();

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
    
    BTreeMap::new().insert("404_message".to_string(), message.to_string());
    let mut data = BTreeMap::new();
    data.insert("404_message".to_string(), message.to_string());

    let handlebars_output = handlebars.render("404", &data).unwrap();

    rocket::response::status::NotFound(Some(RawHtml(handlebars_output)))
}


#[launch]
fn rocket() -> _ {
    if !Path::new("posts.toml").exists() {
        gen_posts();
    }

    rocket::build().mount("/", routes![index, static_files]).register("/", catchers![not_found])
}