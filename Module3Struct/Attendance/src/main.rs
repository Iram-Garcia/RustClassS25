use std::io::{self, Read, Write};
use std::fs::File;

struct Person {
    name: String,
    student_id: u32,
}

fn reading_from_console() {
    let mut buffer = String::new();

    print!("What's your name? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let name = buffer.trim().to_string();
    buffer.clear();

    print!("What is your SID? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let student_id: u32 = buffer.trim().parse().unwrap();

    let person = Person { name, student_id };
    println!("Hi {}, your SID is: {}!", person.name, person.student_id);
}

struct Config {
    name: String,
    student_id: u32,
}

impl Config {
    fn from_file(path: &str) -> Config {
        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let mut lines = contents.lines();
        let name = lines.next().unwrap().to_string();
        let student_id: u32 = lines.next().unwrap().parse().unwrap();

        Config { name, student_id }
    }
}

fn reading_from_file() {
    println!("Reading from file...");
    let config = Config::from_file("config.txt");
    println!("Name: {}", config.name);
    println!("SID: {}", config.student_id);
}

fn main() {
    reading_from_console();
    reading_from_file();
}
