use std::time::{Duration, SystemTime, UNIX_EPOCH};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Note {
    creation_date: u64,
    title: String,
    content: String,
}

impl Note {

    pub fn new(title: String, content: String) -> Note {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let unix_time: u64 = now.as_secs();
        return Note{creation_date: unix_time, title, content}
    }
    
    pub fn get_creation_date(&self) -> SystemTime {
        let creation_unix_time = Duration::from_secs(self.creation_date);
        return UNIX_EPOCH + creation_unix_time;
    }

    pub fn get_title(&self) -> &String {
        return &self.title;
    }

    pub fn get_content(&self) -> &String {
        return &self.content;
    }

}
