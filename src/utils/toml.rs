use serde::{Serialize, Deserialize};
use std::{collections::HashMap};
use toml;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub posts: HashMap<String, String>
}

pub fn gen_posts() {
    let mut post = HashMap::new();
    post.insert("Hello World".to_string(), "Hello World".to_string());
    let config = Config {
        posts: post
    };

    //write to file
    let posts = toml::to_string(&config).unwrap();
    std::fs::write("posts.toml", posts).unwrap();
}

pub fn get_posts() -> Config {
    let posts_file = std::fs::read_to_string("posts.toml").unwrap();
    let posts: Config = toml::from_str(&posts_file).expect("Invalid posts toml please fix any issues in it or delete it to generate a new one.");
    posts
}

pub fn add_post (title: String, content: String) {
    let mut posts_file = get_posts();
    posts_file.posts.insert(title, content);
    let toml = toml::to_string(&posts_file).unwrap();
    std::fs::write("posts.toml", toml).unwrap();
}