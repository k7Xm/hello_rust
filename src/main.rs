use std::fmt;

fn describe<T>(val: &T) 
where
    T: fmt::Debug + fmt::Display,
{
    println!("Debug: {:?}", val);
    println!("Display: {}", val);
}

fn main() {
    describe(&42);
    describe(&"hello");
    describe(&String::from("world"));
}