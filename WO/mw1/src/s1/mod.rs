/*
Sectin 1 Work here
*/

#![allow(unused)]

use crate::utils::print_with_synthwave_gradient;
use yansi::Paint;

pub fn s1main() {
    test_func1();
}

// Test function
fn test_func1() {
    print_with_synthwave_gradient("Section 1".to_string());
    println!("{}", "Booty".red());
}

/////////////// Section 1 Work starts here ///////////////

// Struct with vector
#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

fn s1_fn() {
    // defining a vector of strings which are called bindings
    //  Another way of declaring vector - Vec::new()
    let deck = Deck { cards: vec![] };
    println!("Deck: {:?}", deck);
}
