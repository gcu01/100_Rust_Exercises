mod traits1;
use crate::traits1::{IsEven};

fn main() {
    println!("test");
    let a:u32 = 4;
    println!(" 4 is even? {}", a.is_even());
}