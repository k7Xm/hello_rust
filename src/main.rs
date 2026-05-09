fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        return Err(String::from("Cannot divide by Zero."));
    }
    Ok(a / b)
}

fn divide_twice(a: i32, b: i32, c: i32) -> Result<i32, String> {
    let first = divide(a, b)?;
    let second = divide(first, c)?;
    Ok(second)
}

fn main() {
    println!("{:?}", divide_twice(100, 2, 5));
    println!("{:?}", divide_twice(100, 0, 5));
    println!("{:?}", divide_twice(100, 2, 0));
}