use rand::thread_rng;
use rand::seq::SliceRandom;
use crate::models::card::Card;
use std::io;

fn read_guess() -> u32 {
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    let guess: u32 = guess.trim().parse().expect("Number needed");

    return guess;
}

pub fn simple_scheduler(cards: &mut Vec<Card>) {
    /*
     * A simple scheduler that adds difficult (3) cards
     * at the back of the queue to continuously retest
     * you until you get it right (1 or 2).
     */
    cards.shuffle(&mut thread_rng());
    let mut queue: Vec<&Card> = Vec::new();
    for card in cards.iter() {
        queue.push(card);
    }

    while queue.len() > 0 {
        let card = queue.pop().unwrap();
        println!("---\n{}\n---", card.title);
        println!("Easy: 1, Normal: 2, Tough: 3");

        let mut guess = read_guess();
        while guess < 1 || guess > 3 {
            println!("Choose between 1, 2, or 3");
            guess = read_guess();
        }

        if guess == 3 {
            queue.insert(0, card);
        }

        println!("\n---\n{}:\n{}", card.title, card.body);
    }
}
