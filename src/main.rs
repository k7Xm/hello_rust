mod math;

use math::{add, divide, multiply, subtract};

fn main() {
    println!("{}", add(7.0, 3.0));
    println!("{}", multiply(7.0, 4.0));
    println!("{}", subtract(7.0, 0.0));

    match divide(10.0, 2.0) {
        Ok(v) => println!("{}", v),
        Err(e) => println!("Error, {}", e),
    }

    match divide(10.0, 0.0) {
        Ok(v) => println!("{}", v),
        Err(e) => println!("Error, {}", e),
    }
}
