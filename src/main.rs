fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(a / b)
    }
}

fn divide_twice(n: f64, d1: f64, d2: f64) -> Result<f64, String> {
    let r1 = match divide(n, d1) {
        Ok(v) => v,
        Err(e) => return Err(e),
    };
    println!("Phase1 success, r1 is {}.", r1);

    let r2 = match divide(r1, d2) {
        Ok(v) => v,
        Err(e) => return Err(e),
    };

    println!("Phase2 success, r2 is {}.", r2);

    return Ok(r2);
}

fn main() {
    let n = 100.0;
    let d1 = 2.0;
    let d2 = 5.0;

    match divide_twice(n, d1, d2) {
        Ok(v) => println!("The value is {}", v),
        Err(e) => println!("{}", e),
    }
}
