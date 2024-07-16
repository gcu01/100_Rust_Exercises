mod traits1;
//mod traits2;
//mod traits3;
//mod traits4;
mod traits5;
use crate::traits1::{IsEven};

fn main() {
    println!("test");
    let a:u32 = 4;
    println!(" 4 is even? {}", a.is_even());
}