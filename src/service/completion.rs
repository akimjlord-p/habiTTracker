use chrono::{Local, NaiveDateTime};


pub struct Completion{
    pub completed_at: NaiveDateTime,
    pub note: Option<String>
}


impl Completion{
    pub fn new(note: Option<String>) -> Self{
        Self {
            completed_at: Local::now().naive_local(),
            note 
            }
    }
}