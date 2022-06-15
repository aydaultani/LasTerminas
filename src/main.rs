use std::{collections::HashMap, vec, process::exit};
use rand::seq::SliceRandom;
use colored::*;

fn init_cards(cards: &mut HashMap<String, i32>) {
    cards.insert("1".to_string(), 1);
    cards.insert("2".to_string(), 2);
    cards.insert("3".to_string(), 3);
    cards.insert("4".to_string(), 4);
    cards.insert("5".to_string(), 5);
    cards.insert("6".to_string(), 6);
    cards.insert("7".to_string(), 7);
    cards.insert("8".to_string(), 8);
    cards.insert("9".to_string(), 9);
    cards.insert("10".to_string(), 10);
    cards.insert("K".to_string(), 10);
    cards.insert("Q".to_string(), 10);
    cards.insert("J".to_string(), 10);
    cards.insert("A".to_string(), 10);
}

fn start_bj_game(card_list: Vec<String>, cards: HashMap<String, i32>) {
    let card1 = card_list.choose(&mut rand::thread_rng()).expect("Card random choice failed");
    let card2 = card_list.choose(&mut rand::thread_rng()).expect("Card random choice failed");
    let mut value = cards.get(card1).expect("Getting Card value failed") + cards.get(card2).expect("Getting Card value failed");
    let mut hand = vec![card1, card2];

    println!("{} {:?}", "Your Cards:".bold().green(), hand);
    println!("{} {}", "Current Value:".bold().green() , value);
    loop {
        if value > 21 {
            break
        }
        let mut line = String::new();
        println!("{} ", "Hit [H] or Stand [S]:".bold().blue());
        std::io::stdin().read_line(&mut line).unwrap();

        if line.trim() == "H" {
            let new = card_list.choose(&mut rand::thread_rng()).expect("Card random choice failed");
            hand.push(new);
            value += cards.get(new).unwrap();
            println!("{} {:?}", "Your Cards:".bold().green(), hand);
            println!("{} {}", "Current Value:".bold().green() , value);
        }
        else if line.trim() == "S" {
            
        }
        else {
            println!("{}", "Not found, chance skipped".bold().red())
        }
    }
    if value >= 21 {
        println!("{}", "Imagine losing, couldn't be me.".bold().red())
    }
}

fn black_jack() {
    type Card = HashMap<String, i32>;
    let mut cards: Card = HashMap::new();
    init_cards(&mut cards);
    
    let card_list = vec!["A".to_string(),
                         "K".to_string(),
                         "Q".to_string(),
                         "J".to_string(),
                         "10".to_string(),
                         "9".to_string(),
                         "8".to_string(),
                         "7".to_string(),
                         "6".to_string(),
                         "5".to_string(),
                         "4".to_string(),
                         "3".to_string(),
                         "2".to_string(),
    ];

    start_bj_game(card_list, cards);
}

fn world_bets() {
    todo!()
}

fn main() {
    let mut line = String::new();
    println!("{} ", "BlackJack [BJ] or WorldBets [WB]:".bold().blue());
    std::io::stdin().read_line(&mut line).unwrap();

    if line.trim() == "BJ".to_string() {
        black_jack();
    }
    else if line.trim() == "WB".to_string() {
        world_bets()
    }
    else {
        println!("{}", "Not Found".bold().red());
        exit(0);
    }
}
