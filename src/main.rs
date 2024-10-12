#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    // Creates variables
    let mut random = gen_ran(1, 11); // mut means the variable can be changed
    let mut idx = 0;
    let mut input = String::new();
    const PI: f64 = std::f64::consts::PI;
    // Infinite loop
    loop {
        if random == 10 {
            println!("The variable is 10!");
            break; // Until break
        } else {
            println!("The variable is {}, but not 10!", random);
            random = gen_ran(1, 11);
        };
    };
    // Runs through the variable idx
    while idx <= 10 {
        println!("IDX: {}", idx);
        idx += 1;
    };
    println!("Enter your name!");
    io::stdin().read_line(&mut input)
        .expect("Did not receive input!")
    ;
    println!("Hello, {}", input);
    println!("PI is {}", PI);
}

fn gen_ran(x: i32, y: i32) -> i32 {
    rand::thread_rng().gen_range(x..y)
}
