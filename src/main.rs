pub mod functions;
use std::error::Error;
use std::fs::File;
extern crate csv; 
use csv::Reader;
use std::io;
// use std::collections::HashMap;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate nucleo;
use crate::functions::fns::{FuzzyMatcher, GreedyMatcher, make_names};
use crate::functions::fns::Matcher;

// use nucleo::{Matcher as crateMatcher, Utf32Str};
//star player

#[derive(Deserialize, Debug)]
pub struct Contact{
    name:String,
    phone:String,
}
fn read_contacts(filename: &str) -> Result<Vec<Contact>, Box<dyn Error>> {
    if let Ok(file) = File::open(filename) {
        let mut reader = Reader::from_reader(file);
        let mut contacts = Vec::new();
        for result in reader.deserialize() {
            let contact: Contact = result?;
            contacts.push(contact);
        }
        Ok(contacts)
    } else {
        eprintln!("Error: Could not find '{}'. Make sure it exists.", filename);
        Ok(vec![]) 
}}

pub fn map_vals(digit: char)-> Option<&'static [char]>
{
    //either make mutable map and insert, or use an ext crate for better performance, 
    //since this is just a small one, the match should do fine and be pretty fast too
    match digit {
        '2' =>Some(&['a','b','c']),
        '3' => Some(&['d', 'e', 'f']),
        '4' => Some(&['g', 'h', 'i']),
        '5' => Some(&['j', 'k', 'l']),
        '6' => Some(&['m', 'n', 'o']),
        '7' => Some(&['p', 'q', 'r', 's']),
        '8' => Some(&['t', 'u', 'v']),
        '9' => Some(&['w', 'x', 'y', 'z']),
        _=>None,
    }
}

fn search<T: Matcher>(mut matcher: T, contacts: &Vec<Contact>){
    println!("enter num seq:");
    let mut input=String::new();
    io::stdin().read_line(&mut input).expect("are u even typin bro");
    let input = input.trim();
    if input.is_empty() || !input.chars().all(|c| c.is_digit(10)) {
        println!("Invalid input. Please enter a valid number sequence.");
    }
    
    let possible_names = make_names(input);
    let mut results = Vec::new();
    for name in possible_names {
        results.extend(matcher.find_matches(&name, contacts));
    }
    if results.is_empty() {
        println!("No matches found.");
    } else {
        println!("Matches:");
        for (name, score) in results.iter().take(3) {
            println!("{} (Score: {})", name, score);
        }
    }
}
fn main() {
    println!("Hello, world!");
    let contacts = read_contacts("contacts.csv").expect("Failed to load, just give up");
    loop{
        println!("pick a type :3");
        println!("1.fuzzy match");
        println!("2.greedy fuzzy match (better:D)");
        println!("3.exit.");
        let mut choice =String::new();
        io::stdin().read_line(&mut choice).expect("are u even typin bro");
        let choice=choice.trim();
        match choice {
            "1" =>search(FuzzyMatcher::new(), &contacts),
            "2" =>search(GreedyMatcher::new(), &contacts),
            "3" => break,
            _=> println!("try again :(("),
        }

    // }
    // println!("enter num seq:");
    // let mut input=String::new();
    // io::stdin().read_line(&mut input).expect("are u even typin bro");
    // let input = input.trim;
    // let matcher
}}
