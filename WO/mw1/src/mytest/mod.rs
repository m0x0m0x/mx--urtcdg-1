/*
These will have my functions for testing
*/

use crate::utils::{header, pswg};
use yansi::Paint;

////////// Main Function Calls////////////
pub fn mytest_main() {}

///// Sub Funtions /////

fn struct_impl_test1() {
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

    fn main() {
        let woman1 = Woman {
            boobs: String::from("Big"),
            pussy: String::from("Hairy"),
        };

        println!("{}", woman1.lick());
    }
}
