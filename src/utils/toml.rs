use serde::{Serialize, Deserialize};
use std::{collections::HashMap};
use uuid::Uuid;
use toml;

#[derive(Serialize, Deserialize)]
pub struct Note {
    pub uuid: String,
    pub title: String,
    pub content: String,
}

#[derive(Serialize, Deserialize)]
pub struct NoteFile {
    pub notes: HashMap<String, Note>,
}

pub fn gen_notes() {
    let mut note = HashMap::new();
    let uuid = Uuid::new_v4().to_string();
    note.insert(uuid.clone(), Note {
        uuid: uuid.clone(),
        title: "Hello World".to_string(),
        content: "Hello World".to_string(),
    });
    let config = NoteFile {
        notes: note,
    };

    //write to file
    let notes = toml::to_string(&config).unwrap();
    std::fs::write("posts.toml", notes).unwrap();
}

pub fn get_posts() -> NoteFile {
    let note_file = std::fs::read_to_string("posts.toml").unwrap();
    let notes: NoteFile = toml::from_str(&note_file).expect("Invalid posts toml please fix any issues in it or delete it to generate a new one.");
    notes
}

//TODO: Add a way to add posts
pub fn add_post (title: String, content: String) {
    let mut notes_file = get_posts();
    notes_file.notes.insert(Uuid::new_v4().to_string(), Note {
        uuid: Uuid::new_v4().to_string(),
        title: title,
        content: content,
    });
    let toml = toml::to_string(&notes_file).unwrap();
    std::fs::write("notes.toml", toml).unwrap();
}