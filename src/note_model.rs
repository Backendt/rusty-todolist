use time::OffsetDateTime;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Note {
    creation_date: OffsetDateTime,
    title: String,
    content: String,
}

impl Note {

    pub fn new(title: String, content: String) -> Note {
        let now: OffsetDateTime = OffsetDateTime::now_local()
            .unwrap_or(OffsetDateTime::now_utc());
        return Note{creation_date: now, title, content}
    }
    
    pub fn get_creation_date(&self) -> &OffsetDateTime {
        return &self.creation_date;
    }

    pub fn get_title(&self) -> &String {
        return &self.title;
    }

    pub fn get_content(&self) -> &String {
        return &self.content;
    }

}
