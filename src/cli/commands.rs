use crate::service::utils::{HabitData, HabitPointer};
use thiserror;


pub enum Command {
    New(HabitData),
    Complete(HabitPointer),
    Analytics(HabitPointer),
    Delete(HabitPointer),
    List,
    Quit
}


impl Command {
    pub fn parse(s: &str) -> Result<Self, String>{
        let words: Vec<&str> = s.split_ascii_whitespace().collect();
        if words.len() == 0{
            return Err("Void input".to_string())
        }
        match words[0] {
            "new" => {
                check_n_args(&words, 3)?;
                let habit = HabitData::parse(words[1], words[2])?;
                Ok(Command::New(habit))
            }
            "complete" => {
                check_n_args(&words, 2)?;
                let habit = HabitPointer::parse(&words[1]);
                Ok(Command::Complete(habit))
            },
            "delete" => {
                check_n_args(&words, 2)?;
                let habit = HabitPointer::parse(&words[1]);
                Ok(Command::Delete(habit))
            },
            "analytics" => {
                check_n_args(&words, 2)?;
                let habit = HabitPointer::parse(&words[1]);
                Ok(Command::Analytics(habit))
            },
            "list" => {
                Ok(Command::List)
            }
            "quit" => {
                Ok(Command::Quit)
            }
            _ => Err("Unexpected command".to_string())
        }
    }
}

fn check_n_args(words: &Vec<&str>, n: i64) -> Result<(), String> {
    if words.len() != n as usize{
        return Err(format!("Expected {} args, get {}", n-1, (words.len() - 1)).to_string())
    }
    else{
        Ok(())
    }
}