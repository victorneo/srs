use models::card::Card;
use models::dir::Directory;
use std::env;
use std::io;
mod loaders;
mod models;
mod schedulers;


fn read_dir_index(max: usize) -> usize {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let mut dir_index: usize = input.trim().parse().expect("Number needed");

    while dir_index < 1 || dir_index > max {
        println!("Invalid entry");
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        dir_index = input.trim().parse().expect("Number needed");
    }

    return dir_index;
}


fn main() {
    let mut cards: Vec<Card> = Vec::new();
    let mut dirs: Vec<Directory> = Vec::new();

    // Default path to look for cards is current directory
    // Allow users to specify other directories
    let args: Vec<String> = env::args().collect();
    let mut path = "./";

    if args.len() > 1 {
        path = &args[1];
    }

    // Load non-hidden directories
    loaders::load_directories(path, &mut dirs);
    if dirs.len() == 0 {
        println!("No directories in given path");
        return;
    }

    // Display subdirectories in given directory to load cards from
    let mut count = 1;
    for dir in dirs.iter() {
        println!("{}: {}", count, dir.file_name);
        count += 1;
    }

    let dir_index = read_dir_index(dirs.len());
    let dir = dirs.get(dir_index - 1).unwrap();

    // Load cards from chosen directory, run scheduler on them
    loaders::load_cards(&dir.path, &mut cards);
    println!("Loaded {} cards", cards.len());

    // Future: customize scheduler?
    schedulers::simple_scheduler(&mut cards);
}
