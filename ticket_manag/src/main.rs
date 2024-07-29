//mod manag1;
//mod manag2;
//mod manag3;
mod manag4;
//use crate::manag1::{Weekday, WeekTemperature};

//use crate::manag1::{Weekday, WeekTemperature};

fn main() {
    println!("Hello, world!");

    let v:Vec<i32> = vec![5,6,98,3];
    for n in &v { 
        println!("{}", n);
    }

    for n in v.iter() { 
        println!("{}", n);
    }
    println!("{}", v[0]);
}
