/*
Section 3 - Bank Project work
*/

#[allow(unused)]
use crate::utils::{header, pswg};

//////// /// Main function call ////////////
pub fn bank1_main() {
    sb1();
}

////// Sub functions here //////////

// Test function
fn sb1() {
    pswg("Sub Bank 1".to_string());
    header("Sub Function test")
}

//// Sec1 - Bank Project Work Here //////////

// * Main struct
struct Account {
    balance: u32,
    id: i32,
    holder: String,
}
