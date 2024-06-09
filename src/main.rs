// use match_case::matchcase;
// use match_case::test_match_array;
// use anonymus_fn::{anoy, anoy_bin};
// use loop_ers::for_loop;
use crate::my_struct::call_stack;
// use crate::ownership::call_stack;
// use crate::check::call_stack;

// pub mod  loop_ers;
// pub mod  match_case;
// pub mod  anonymus_fn;
pub mod my_struct;
// pub mod check;
// pub mod ownership;
fn main() {
    call_stack();
}

// @important
// println! is a macro that prints a string to the screen:
// String::new, a function that returns a new instance of a String.
