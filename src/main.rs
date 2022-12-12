//use crate::day1;
use std::fs;
use std::io;

mod day1;

type challenge = fn(&String) -> String;

struct Day {
    pub name: String,
    pub first: challenge,
    pub second: challenge,
}

fn null_second(_: &String) -> String {
    "".to_string()
}

impl Day {
    fn new(name: String, first: challenge, second: challenge) -> Self {
        Self {
            name,
            first,
            second,
        }
    }
}

fn main() {
    let days = vec![
        Day::new("Calorie Counting".into(), day1::first, day1::second),
        // Future days go here
    ];
    for (i, day) in days.iter().enumerate() {
        let file_path = format!("./src/day{0}.txt", i + 1);
        let res = fs::read_to_string(&file_path);
        match res {
            Ok(contents) => {
                let res1 = (day.first)(&contents);
                let res2 = (day.second)(&contents);
                println!("{} => {}, {}", day.name, res1, res2);
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
