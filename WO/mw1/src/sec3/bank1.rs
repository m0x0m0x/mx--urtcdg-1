/*
Section 3 - Bank Project work
*/

#[allow(dead_code)]
// Impors
use crate::utils::{header, pswg};

//////// /// Main function call ////////////
pub fn bank1_main() {
    sb_main_bank();
}

////// Sub functions here //////////

// Test function
fn sbtest() {
    pswg("Sub Bank 1".to_string());
    header("Sub Function test")
}

//// Sec1 - Bank Project Work Here //////////

// * Main struct
#[derive(Debug)]
#[allow(dead_code)]
struct Account {
    balance: u32,
    id: u32,
    holder: String,
}

#[allow(dead_code)]
impl Account {
    fn new(id: u32, holder: String) -> Self {
        Account {
            id,
            holder,
            balance: 0,
        }
    }
}

// Bank Struct that holds the Account structs
#[derive(Debug)]
#[allow(dead_code)]
struct Bank {
    accounts: Vec<Account>,
}

#[allow(dead_code)]
impl Bank {
    fn new() -> Self {
        Bank { accounts: vec![] }
    }
}

fn sb_main_bank() {
    pswg("Main Bank Function".to_string());
}
