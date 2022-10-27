use std::io;
use rand::thread_rng;
use rand::seq::SliceRandom;
use models::card::Card;
use models::dir::Directory;
mod models;
mod loaders;


fn main() {
    let mut cards : Vec<Card> = Vec::new();
    let mut dirs : Vec<Directory> = Vec::new();

    loaders::load_directories("./samples", &mut dirs);

    if dirs.len() == 0 {
        println!("No directories in given path");
        return;
    }

    let mut count = 1;
    let mut input = String::new();
    for dir in dirs.iter() {
        println!("{}: {}", count, dir.file_name);
        count += 1;
    }
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input : usize = input.trim().parse().expect("Number needed");

    if input < 1 || input > dirs.len() {
        println!("Invalid entry");
        return;
    }

    let dir = dirs.get(input-1).unwrap();

    let mut easy_count = 0;
    let mut normal_count = 0;
    let mut tough_count = 0;

    // Load cards from chosen directory
    loaders::load_cards(&dir.path, &mut cards);
    println!("Loaded {} cards", cards.len());
    cards.shuffle(&mut thread_rng());

    for card in cards.iter() {
        println!("---\n{}\n---", card.title);
        println!("Easy: 1, Normal: 2, Tough: 3");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess : u32 = guess.trim().parse().expect("Number needed");

        match guess {
            1 => easy_count += 1,
            2 => normal_count += 1,
            3 => tough_count += 1,
            _ => println!("Invalid input"),
        }

        println!("Answer: {}", card.body);
    }

    println!("---\nSummary");
    println!("Easy: {}, Normal: {}, Tough: {}", easy_count, normal_count, tough_count);
}
