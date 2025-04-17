/*
Sectin 1 Work here
*/

#![allow(unused)]

use crate::utils::header;
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

fn s1_fn() {
    // Printing the header text
    header("Section 1 - Structs and Vectors");

    // defining a vector of strings which are called bindings
    //  Another way of declaring vector - Vec::new()
    let deck = Deck { cards: vec![] };
    println!("Deck: {:#?}", deck);
}
