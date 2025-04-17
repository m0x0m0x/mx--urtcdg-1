/*
Sectin 1 Work here
*/

#![allow(unused)]

use crate::utils::{header, pswg};
use yansi::Paint;

pub fn s1main() {
    test_func1();
}

// Test function
fn test_func1() {
    s1_fn();
}

/////////////// Section 1 Work starts here ///////////////

// Struct with vector
#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

// Inherent Implemenation
impl Deck {
    fn new() -> Self {}
}

fn s1_fn() {
    // Printing the header text
    pswg("Section 1 - Structs and Vectors".to_string());

    // defining a vector of strings which are called bindings
    //  Another way of declaring vector - Vec::new()

    // list of suits
    let suits = ["SS", "HH", "AA", "DD"];
    // List of values
    let values = ["Ace", "Two", "Three"];

    // Empty vector of cards
    let mut cards = vec![];

    // Example of reassignment
    // cards = vec![];

    // Double nested for loops - to generate combinations
    for suit in suits {
        for value in values {
            let card = format!("{} of {}", value, suit);
            cards.push(card);
        }
    }

    let deck = Deck { cards };
    println!("Deck: {:#?}", deck.green());
}
