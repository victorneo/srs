use std::fs;
use std::fs::File;
use std::io::prelude::*;
use crate::models::card::Card;
use crate::models::dir::Directory;


pub fn load_cards(dir_name: &str, cards: &mut Vec<Card>) {
    let paths = fs::read_dir(dir_name).unwrap();

    // Get filenames in each directory
    for path in paths {
        // Ignore .DS_Store
        let p = path.unwrap();
        let fname = p.file_name();
        if fname == ".DS_Store" {
            println!("Note: Found .DS_Store, skipping");
            continue;
        }

        let metadata = p.metadata().unwrap();
        if metadata.is_dir() {
            continue;
        }

        let mut file = File::open(p.path()).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let card = Card {
            title: String::from(fname.to_str().unwrap().trim_end_matches(".md")),
            body: contents,
        };
        cards.push(card);
    }
}


pub fn load_directories(dir_name: &str, dirs: &mut Vec<Directory>) {
    let paths = fs::read_dir(dir_name).unwrap();

    for path in paths {
        let p = path.unwrap();
        let metadata = p.metadata().unwrap();
        if metadata.is_dir() {
            let dir = Directory {
                file_name: String::from(p.file_name().to_str().unwrap()),
                path: String::from(p.path().to_str().unwrap()),
            };
            dirs.push(dir);
        }
    }
}