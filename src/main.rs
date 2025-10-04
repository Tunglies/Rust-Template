use rust_template::{add, subtract};

fn main() {
    println!("Rust Template Example");
    println!("=====================");

    let a = 10;
    let b = 5;

    println!("{} + {} = {}", a, b, add(a, b));
    println!("{} - {} = {}", a, b, subtract(a, b));
}
