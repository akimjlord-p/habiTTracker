use crate::service::habit::Habit;
use serde::{Deserialize, Serialize};
use std::fs;
use crate::service::utils::HabitPointer;
use thiserror::Error;



#[derive(Debug, Error)]
pub enum DbError{
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Habit '{0}' already exists")]
    HabitAlreadyExist(String),

    #[error("Incorrect habit name '{0}'")]
    IncorrectHabitName(String),

    #[error("Index out of bounds '{0}' ")]
    IndexOutOfBounds(String)

}

#[derive(Deserialize, Serialize)]
pub struct Storage{
    habits: Vec<Habit>
}


impl Storage{
    pub fn generate() -> Result<Self, DbError>{
        let file = fs::read_to_string("storage.json");
        match file {
            Ok(file) => Ok(serde_json::from_str(&file)?),
            Err(_) => {
                Ok(Self{
                    habits: Vec::new()
                })
            }
        }
    }


    pub fn save(&self) -> Result<(), DbError> {
        let json = serde_json::to_string_pretty(&self)?;
        fs::write("storage.json", json)?;
        Ok(())
    }

    pub fn add_habit(&mut self, habit: Habit) -> Result<(), DbError>{
        if let Some(_) = self.habits.iter().find(|h| h.name == habit.name){
            return Err(DbError::HabitAlreadyExist(habit.name));
        }
        self.habits.push(habit);
        self.save()?;
        Ok(())
    }

    pub fn delete_habit(&mut self, habit_pointer: HabitPointer) -> Result<usize, DbError>{
        match habit_pointer {
            HabitPointer::Name(name) => self.delete_by_name(name),
            HabitPointer::Number(idx) => self.delete_by_idx(idx),
        }
    }

    fn delete_by_name(&mut self, habit_name: String) -> Result<usize, DbError>{
        if let None = self.habits.iter().find(|h| h.name == habit_name){
            return Err(DbError::IncorrectHabitName(habit_name.to_string()));
        }
        let start_len =self.habits.len();
        self.habits.retain(|h| h.name != habit_name);
        self.save()?;
        Ok(start_len - self.habits.len())
    }

    fn delete_by_idx(&mut self, idx: i64) -> Result<usize, DbError>{
        if self.habits.get(idx as usize).is_none() {
            return Err(DbError::IndexOutOfBounds(idx.to_string()));
        }        
        let start_len = self.habits.len();
        self.habits.remove(idx as usize);
        self.save()?;
        Ok(start_len - self.habits.len())
    }

    pub fn get_habits(&self) -> &[Habit]{
        &self.habits
    }

    pub fn get_mut_habit(&mut self, habit_pointer: HabitPointer) -> Result<&mut Habit, DbError>{
        match habit_pointer {
            HabitPointer::Name(name) => self.get_mut_habit_by_name(name),
            HabitPointer::Number(idx) => self.get_mut_habit_by_idx(idx),
        }

    }

    fn get_mut_habit_by_name(&mut self, habit_name: String) -> Result<&mut Habit, DbError>{

        self.habits.iter_mut().find(|h| h.name == habit_name)
        .ok_or_else(||DbError::IncorrectHabitName(habit_name.to_string()))
    }

    fn get_mut_habit_by_idx(&mut self, idx: i64) -> Result<&mut Habit, DbError>{
        self.habits.get_mut(idx as usize).ok_or_else(||DbError::IndexOutOfBounds(idx.to_string()))
    }
}