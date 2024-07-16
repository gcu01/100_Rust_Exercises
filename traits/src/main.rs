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

    println!("size of a string, fat pointer with 3 values (i.e., pointer, length, capacity) = {:?}", std::mem::size_of::<String>());
    println!("size of a usize = {:?}", std::mem::size_of::<usize>());
    println!("size of a &str, fat pointer with 2 values (i.e., pointer, size)  = {:?}", std::mem::size_of::<&str>());
}