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
}

impl Woman {
    fn lick(&self) -> String {
        format!("Woman boobs {} and smelly {}", self.boobs, self.pussy)
    }
}

fn struct_impl_test1() {
    pswg("Struct and Implementation Test".to_string());

    let woman1 = Woman {
        boobs: String::from("Big"),
        pussy: String::from("Hairy"),
    };

    println!("{}", woman1.lick());
}
