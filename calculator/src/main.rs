use std::env;
mod integers;
use integers::{factorial, print_max_min_i32};

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    print_max_min_i32();
    factorial(3);
    factorial(20);
}
