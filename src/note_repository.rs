use crate::note_model::Note;
use std::fs::{read, write};
use bincode::{deserialize, serialize};
use regex::Regex;
use std::env;

fn get_notes_filepath() -> String {
    let home_env: String = env::var("HOME").expect("HOME environment variable is not set");
    return home_env + "/.todos";
}

fn write_notes(notes: Vec<Note>) {
    let serialized_note: Vec<u8> = serialize(&notes).unwrap();
    let notes_filepath = get_notes_filepath();
    write(notes_filepath, serialized_note).expect("Could not write note to todos file");
}

pub fn get_notes() -> Vec<Note> {
    let notes_filepath = get_notes_filepath();
    let note_file = read(notes_filepath);

    return match &note_file {
        Ok(notes_bytes) => deserialize(&notes_bytes[..]).unwrap(),
        Err(..) => Vec::new(),
    }
}

pub fn get_notes_by_title_regex(title: &String) -> Vec<Note> {
    let re = Regex::new(title).expect("Invalid title regex");
    let mut matching_notes: Vec<Note> = Vec::new();

    let notes: Vec<Note> = get_notes();
    for note in notes {
        if re.is_match(&note.get_title()) {
            matching_notes.push(note);
        }
    }

    return matching_notes;
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

pub fn add_note(note: Note) {
    let mut note_list: Vec<Note> = get_notes();
    note_list.push(note);
    write_notes(note_list);
}

pub fn delete_note_by_title(title: &String) -> bool {
    let mut notes: Vec<Note> = get_notes();
    for(note_index, note) in notes.iter().enumerate() {
        if note.get_title() == title {
            notes.remove(note_index);
            write_notes(notes);
            return true;
        }
    }

    return false;
}

pub fn delete_note_by_regex_title(title: &String) -> u16 {
    let re = Regex::new(title).expect("Invalid title regex");

    let mut deleted_notes_amount: u16 = 0;

    let notes: Vec<Note> = get_notes();
    let new_notes = notes.into_iter()
        .filter(|note| {
            let matches: bool = re.is_match(&note.get_title());
            if matches {
                deleted_notes_amount += 1;
            }
            return !matches;
        })
        .collect();

    write_notes(new_notes);
    return deleted_notes_amount;
}
