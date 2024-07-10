mod ticket;
use ticket::*;

fn main() {
    println!("Hello, world!");
    let my_o = Order{price:2, quantity:1};
    println!("function call {}", Order::my_fct(5));
}
