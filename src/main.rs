/*
* !!!This is not my own work!!!
* This project is a follow along with a course by Ahmed Elsakka on Udemy
*/
use math_utils::operations::{add::add, multiply::multiply};

fn main() {
    // create some integer varaibles
    let a = 3;
    let b = 4;

    println!("Addition: {}", add(a, b));
    println!("Multiplication: {}", multiply(a, b));
}
