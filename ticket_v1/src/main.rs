mod ticket;
use crate::ticket::{Order, ticket_t1::Ticket};

fn main() {
    println!("Hello, world!");
    let my_o = Order{price:2, quantity:1};
    println!("function call {}", Order::my_fct(5));

    let t:Ticket = Ticket::new("a".to_string(), "v".to_string(), "Done".to_string());
    println!("size of i32 = {}", std::mem::size_of::<i32>());
    println!("size of usize {}", std::mem::size_of::<usize>());
    println!("size of usize {}", std::mem::size_of::<String>());
    println!("size of usize {}", std::mem::size_of::<&String>());
    println!("size of usize {}", std::mem::size_of::<&mut String>());
}
