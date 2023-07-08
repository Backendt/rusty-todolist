use crate::note_model::Note;
use std::fs::{read, write};
use bincode::{deserialize, serialize};
use regex::Regex;

const NOTES_FILEPATH: &str = "~/.todos";

fn write_notes(notes: Vec<Note>) {
    let serialized_note: Vec<u8> = serialize(&notes).unwrap();
    write(NOTES_FILEPATH, serialized_note).expect("Could not write note to todos file");
}

pub fn get_notes() -> Vec<Note> {
    let note_file = read(NOTES_FILEPATH);

    return match &note_file {
        Ok(notes_bytes) => deserialize(&notes_bytes[..]).unwrap(),
        Err(..) => Vec::new(),
    }
}

pub fn get_note_by_title_regex(title: &String) -> Option<Note> {
    let re = Regex::new(title).expect("Invalid title regex");

    let notes: Vec<Note> = get_notes();
    for note in notes {
        if re.is_match(&note.get_title()) {
            return Option::Some(note);
        }
    }

    return Option::None;
}

pub fn get_note_by_title(title: &String) -> Option<Note> {
    let notes: Vec<Note> = get_notes();
    for note in notes {
        if note.get_title().contains(title) {
            return Option::Some(note);
        }
    }
    return Option::None;
}

pub fn add_note(title: String, content: String) {
    let note: Note = Note::new(title, content);
    let mut note_list: Vec<Note> = get_notes();
    note_list.push(note);
    write_notes(note_list);
}

pub fn delete_note_by_title(title: &String) {
    let mut notes: Vec<Note> = get_notes();
    for (note_index, note) in notes.iter().enumerate() {
        if note.get_title() == title {
            notes.remove(note_index);
            write_notes(notes);
            return;
        }
    }
}

pub fn delete_note_by_regex_title(title: &String) {
    let re = Regex::new(title).expect("Invalid title regex");

    let mut notes: Vec<Note> = get_notes();
    for(note_index, note) in notes.iter().enumerate() {
        if re.is_match(&note.get_title()) {
            notes.remove(note_index);
            write_notes(notes);
            return;
        }
    }
}
