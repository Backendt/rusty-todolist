use crate::note_model::Note;
use crate::note_repository;
use time::format_description::parse;

fn print_note(note: Note) {
    let time_format = parse("[day]/[month]/[year] [hour]:[minute]").unwrap();
    let creation_time = note.get_creation_date();
    let time_date: String = creation_time.format(&time_format)
        .unwrap_or(String::from("Unknown time"));

    let title = note.get_title();
    let content = note.get_content();
    println!("####### {title} #######\n{time_date}\n\n{content}");
}

pub fn list_notes() {
    let notes: Vec<Note> = note_repository::get_notes();
    if notes.is_empty() {
        println!("No notes.");
        return;
    }

    for note in notes {
        let title = note.get_title();
        println!("##### {title} #####");
    }
}

pub fn read_note(title: &String, is_regex: bool) {
    let mut notes_to_read: Vec<Note> = Vec::new();
    
    if is_regex {
        let regex_title: String = format!("^{title}$");
        let mut notes: Vec<Note> = note_repository::get_notes_by_title_regex(&regex_title);
        notes_to_read.append(&mut notes);
    } else {
        let optinal_note: Option<Note> = note_repository::get_note_by_title(title);
        optinal_note.map(|note| notes_to_read.push(note));
    }
    
    if notes_to_read.is_empty() {
        println!("No notes.");
        return;
    }

    for note in notes_to_read {
        print_note(note);
    }
}

pub fn add_note(title: &String, content: &String) {
    let note: Note = Note::new(title.clone(), content.clone());
    note_repository::add_note(note);
    println!("Note added.");
}

pub fn delete_note(title: &String, is_regex: bool) {
    let mut amount_deleted = 0;
    if is_regex {
        let regex_title: String = format!("^{title}$");
        amount_deleted = note_repository::delete_note_by_regex_title(&regex_title);
    } else {
        let note_was_deleted: bool = note_repository::delete_note_by_title(title);
        if note_was_deleted {
            amount_deleted = 1;
        }
    }

    println!("Deleted {amount_deleted} note(s).");
}
