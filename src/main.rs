#![allow(unused_parens)]
#![allow(dead_code)]
#[allow(unused_imports)]

mod utils;
mod day1 {
    pub mod main; // Link the `a.rs` file as a submodule
}
use day1::main::run;

// mod day2 {
//     pub mod main; // Link the `a.rs` file as a submodule
// }
// use day2::main::run;

fn main() {
    run(); // Call your function here
}
