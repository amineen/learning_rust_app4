#![allow(unused)]
extern crate chrono;

use chrono::{DateTime, Local, NaiveDateTime, Utc};
fn main() {
    let current_time: DateTime<Utc> = Utc::now();

    let iso_time_string = current_time.to_rfc3339();

    let datetime_array = iso_time_string.split('T').collect::<Vec<&str>>();

    let s = "Aaron Mineen";
    let binaries = string_to_binaries(s);
    println!("{:?}", binaries);

    let binary_vector = string_to_binary_vector(s);
    println!("{:?}", binary_vector);

    let string = binary_vector_to_string(&binary_vector);
    println!("{:?}", string);
    println!("{:?}", binary_vector);

    let dog = Dog {
        name: String::from("Rusty"),
    };
    let cat = Cat {
        name: String::from("Banny"),
    };

    dog.speak();
    cat.speak();
}

fn string_to_binaries(s: &str) -> String {
    s.chars()
        .map(|c| format!("{:08b}", c as u8))
        .collect::<Vec<String>>()
        .join("")
}

fn string_to_binary_vector(s: &str) -> Vec<String> {
    s.chars()
        .map(|c| format!("{:08b}", c as u8))
        .collect::<Vec<String>>()
}

fn binary_vector_to_string(binary_vector: &[String]) -> String {
    binary_vector
        .iter()
        .map(|binary| {
            let decimal = i32::from_str_radix(binary, 2).unwrap();
            char::from_u32(decimal as u32).unwrap()
        })
        .collect()
}

trait Speak {
    fn speak(&self);
}

struct Dog {
    name: String,
}

struct Cat {
    name: String,
}

impl Speak for Dog {
    fn speak(&self) {
        println!("{} says woof!", self.name);
    }
}

impl Speak for Cat {
    fn speak(&self) {
        println!("{} says meow!", self.name);
    }
}
