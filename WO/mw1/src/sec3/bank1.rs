/*
Section 3 - Bank Project work
*/

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

struct Account {
    balance: int32,
    id: usize,
}
