/*
Note the mod keyword will have the file which contains the function
.
├── mod.rs
└── w1.rs
- Note the file structure
- If there was w2.rs , Then the declaratin would be
pub mod w2
- Then in main.rs , you will call the function within w2.rs
- For example the function inside w1.rs is w1_sec3_main, which is bein called in main.rs for execution
*/

pub mod w1;
