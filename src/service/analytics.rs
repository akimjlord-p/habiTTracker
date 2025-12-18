use super::habit::Habit;
use chrono::{Duration, Local, NaiveDateTime};
use super::utils::is_in_range;


pub fn num_of_completions(habit: &Habit, start: &NaiveDateTime, end: &NaiveDateTime) -> i64{
    let completions = &habit.completions;
    let mut counter = 0;
    for completion in completions{
        if is_in_range(&completion.completed_at, &start, &end){
            counter += 1;
        }
    }
    counter
}


pub fn percentage_of_completion_throughout(habit: &Habit) -> i64 {
    let start = habit.created_at;
    let end = Local::now().naive_local();
    let interval = end - start;

    let interval_secs = interval.num_seconds();
    let freq_secs = habit.frequency.num_seconds();

    if interval_secs <= 0 || freq_secs <= 0 {
        return 0;
    }

    let expected_count = interval_secs / freq_secs;
    if expected_count == 0 {
        return 0;
    }

    let completions = num_of_completions(habit, &start, &end) as i64;
    (completions * 100 / expected_count).min(100)
}


pub fn percentage_of_completion_last_week(habit: &Habit) -> i64 {
    let end = Local::now().naive_local();

    let mut start = end - Duration::days(7);
    if start <= habit.created_at{
        start = habit.created_at;
    }
    let completions = num_of_completions(&habit, &start, &end);
    
    let expected_count = Duration::days(7).num_seconds() / habit.frequency.num_seconds();
    
    if expected_count == 0 {
        return 0;
    }
    
    (completions as i64 * 100 / expected_count).min(100)
}