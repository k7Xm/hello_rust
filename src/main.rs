enum FactoryError {
    EmptyInput,
    InvalidNumber(String),
    OverScale(f64),
}

fn factory_process(s: &str) -> Result<f64, FactoryError> {
    if s.is_empty() {
        return Err(FactoryError::EmptyInput);
    }

    let num = s.parse::<f64>()
        .map_err(|e| FactoryError::InvalidNumber(e.to_string()))?;

    if num > 100.0 {
        return Err(FactoryError::OverScale(num));
    }
    Ok(num / 2.0)
}

fn main() {
    let s = "999d";

    match factory_process(s) {
        Ok(v) => println!("The value is {}", v),
        Err(FactoryError::EmptyInput) => println!("Imput Something."),
        Err(FactoryError::InvalidNumber(s)) => println!("{}", s),
        Err(FactoryError::OverScale(s)) => println!("{:?} is too big.", s),
    }
}