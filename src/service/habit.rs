use chrono::{NaiveDateTime, Local, Duration};
use crate::service::completion::Completion;

pub struct Habit{
    name: String,
    description: Option<String>,
    pub completions: Vec<Completion>,
    pub created_at: NaiveDateTime,
    archived_at: Option<NaiveDateTime>,
    is_archived: bool,
    pub frequency: Duration
}


impl Habit{
    pub fn new(name: String, description: Option<String>, frequency: Duration) -> Self{
        Self{
            name,
            description,
            completions: Vec::new(),
            created_at: Local::now().naive_local(),
            archived_at: Option::None,
            is_archived: false,
            frequency
        }
    }


    pub fn complete(&mut self, note: Option<String>){
        self.completions.push(Completion::new(note));
    }

    pub fn archive(&mut self){
        self.is_archived = true;
        self.archived_at = Some(Local::now().naive_local());
    }

    pub fn is_archived(self) -> bool{
        self.is_archived
    }


}