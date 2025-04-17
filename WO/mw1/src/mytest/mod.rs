/*
These will have my functions for testing
*/

#![allow(unused)]

use crate::utils::{header, pswg};
use yansi::Paint;

////////// Main Function Calls////////////
pub fn mytest_main() {
    struct_impl_test1();
}

///// Sub Funtions /////

/*
Testing struct and implementation
*/

#[derive(Debug)]
struct Woman {
    boobs: String,
    pussy: String,
    ass: String,
}

impl Woman {
    fn lick(&self) -> String {
        format!(
            "Woman boobs {} and smelly {} giant {}",
            self.boobs, self.pussy, self.ass
        )
    }
}

fn struct_impl_test1() {
    pswg("Struct and Implementation Test".to_string());

    let woman1 = Woman {
        boobs: String::from("Big"),
        pussy: String::from("Hairy"),
        ass: String::from("fluddy"),
    };

    println!("{}", woman1.lick());
}
