/*
Note the mod keyword will have the file which contains the function
.
├── mod.rs
└── w1.rs
- Note the file structure
- If there was w2.rs , Then the declaratin would be
pub mod w2
- Then in main.rs , you will call the function within w2.rs
*/

pub mod w1;
