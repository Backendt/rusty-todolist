use crate::note_model::Note;
use crate::note_repository::*;

pub fn list_notes() {
    println!("Listing notes");
}

pub fn read_note(title: &String, is_regex: bool) {
    println!("Reading note {title} (is regex: {is_regex})");
}

pub fn add_note(title: &String, content: &String) {
    println!("Adding note {title}: {content}");
}

pub fn delete_note(title: &String, is_regex: bool) {
    println!("Deleting note(s) {title} (is regex: {is_regex})");
}
