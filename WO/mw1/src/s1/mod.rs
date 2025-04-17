/*
Sectin 1 Work here
*/

use crate::utils::print_with_synthwave_gradient;
use yansi::Paint;

pub fn s1main() {
    func1();
}

fn func1() {
    print_with_synthwave_gradient("Section 1".to_string());
    println!("{}", "Booty".red());
}
