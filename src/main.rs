use std::collections::BTreeMap;
use handlebars::Handlebars;
use rocket::{serde::Serialize, response::content::RawHtml};

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

    let post1 = BlogPost {
        title: "First Post".to_string(),
        content: "This is the content of the first post.".to_string(),
    };

    let post2 = BlogPost {
        title: "Second Post".to_string(),
        content: "This is the content of the second post.".to_string(),
    };

    let mut data = BTreeMap::new();
    data.insert("posts".to_string(), vec![post1, post2]);

    let handlebars_output = handlebars.render("index", &data).unwrap();

    //render as html with css
    RawHtml(handlebars_output)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}