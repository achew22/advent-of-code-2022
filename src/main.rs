//use crate::day1;
use std::fs;
use std::io;

mod day1;

struct Day {
    pub name: String,
    pub f: fn(String) -> String,
}

impl Day {
    fn new(name: String, f: fn(String) -> String) -> Self {
        Self { name, f }
    }
}

fn main() {
    let days = vec![
        Day::new("Calorie Counting".into(), day1::run),
        // Future days go here
    ];
    for (i, day) in days.iter().enumerate() {
        let file_path = format!("./src/day{0}.txt", i + 1);
        let res = fs::read_to_string(&file_path);
        match res {
            Ok(contents) => {
                let res = (day.f)(contents);
                println!("{} => {}", day.name, res);
            }
            Err(e) => {
                println!("Unable to read {file_path}: {e}")
            }
        }
    }

    print!("Please enter the day you want to run: ");

    let mut day = String::new();

    io::stdin()
        .read_line(&mut day)
        .expect("Failed to read line");
}
