mod ticket;
use crate::ticket::{Order, ticket_t1::Ticket};

fn main() {
    println!("Hello, world!");
    let my_o = Order{price:2, quantity:1};
    println!("function call {}", Order::my_fct(5));

    let t:Ticket = Ticket::new("".to_string(), "v".to_string(), "f".to_string());
}
