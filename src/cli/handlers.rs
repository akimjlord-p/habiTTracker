use super::commands::Command;
use crate::service::habit::Habit;
use crate::service::analytics::{num_of_completions, percentage_of_completion_throughout, percentage_of_completion_last_week};
use crate::db::storage::Storage;
use chrono::Local;
use std::io;


pub struct CLi{
    storage: Storage,
    running: bool
}


impl CLi{
    fn execute_command(&mut self, command: Command) -> Result<String, String> {
        match command {
            Command::New(habit_data) => {
                let new_habit = Habit::new(habit_data);
                match self.storage.add_habit(new_habit){
                    Ok(_) => Ok("Habit successfully added".to_string()),
                    Err(_) => Err("Error with saving, try again".to_string()),
                }
            },
            Command::Complete(habit_pointer) => {
                match self.storage.get_mut_habit(habit_pointer){
                    Ok(habit) => {
                        habit.complete(None);
                        self.storage.save();
                        Ok("Completed!:)".to_string())
                    },
                    Err(_) => Err("Error in habit name/number".to_string()),
                }
            },
            Command::Analytics(habit_pointer) => {
                match self.storage.get_mut_habit(habit_pointer) {
                    Ok(habit) => {
                        let mut res = String::new();
                        res += &format!("Num of completions: {} \n", num_of_completions(habit, &habit.created_at, &Local::now().naive_local()));
                        res += &format!("Percentage of completion for the all time: {}\n", percentage_of_completion_throughout(&habit));
                        res += &format!("Percentage of completion for the last week: {}\n", percentage_of_completion_last_week(&habit));
                        Ok(res)

                    },
                    Err(_) => Err("Error in habit name/number".to_string()),
                }
            },
            Command::Delete(habit_pointer) => {
                match self.storage.delete_habit(habit_pointer) {
                    Ok(_) => Ok("Deleted!:(".to_string()),
                    Err(_) => Err("Error in habit name/number".to_string()),
                }
            },
            Command::List => {
                let habits = self.storage.get_habits();

                if habits.is_empty() {
                    return Ok("No habits yet".to_string());
                }

                let mut res = String::new();
                for (i, habit) in habits.iter().enumerate() {
                    res += &format!(
                        "{}. {} (completions: {})\n",
                        i,
                        habit.name,
                        habit.completions.len()
                    );
                }

                Ok(res)
            }
            Command::Quit => {
                self.storage.save();
                self.running = false;
                Ok("Goodbuy!".to_string())
            },
        }
    }


    pub fn initiate() -> Self{
        let storage = Storage::generate().unwrap();
        Self { storage, running: false }
    }

    pub fn run(&mut self){
        self.running = true;
        while self.running{
            let mut input = String::new();
            println!("Enter the command >");
            io::stdin().read_line(&mut input).expect("Error with input in [fn get_input]");
            let command = Command::parse(input.trim());
            match command {
            Ok(command) => {
                match self.execute_command(command) {
                    Ok(msg) => println!("{msg}"),
                    Err(err) => eprintln!("{err}"),
                }
            }
            Err(e) => println!("{e}"),
        }
        }
    }
}