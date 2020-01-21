use svelte_rs::*;
use std::env;

fn main() {
    let a = 2;
    let b = 2;
    println!("{} + {} = {}", a, b, add(a, b));
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}
