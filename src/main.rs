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

    let path = std::env::current_dir().unwrap().to_str().unwrap().to_string();
    
    BTreeMap::new().insert("path".to_string(), path.to_string());
    let mut data = BTreeMap::new();
    data.insert("path".to_string(), path.to_string());

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