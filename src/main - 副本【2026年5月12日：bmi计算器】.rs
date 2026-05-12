enum ErrorType {
    Empty,
    Invalid,
}

struct Calc {
    height: f64,
    weight: f64,
}

#[derive(Debug)]
enum BodyType {
    Standard,
    Fat,
    Slim,
}


fn check_input(h: &str, w: &str) -> Result<Calc, ErrorType> {
    if h.is_empty() || w.is_empty() {
        return Err(ErrorType::Empty);
    }

    let h1 = h.parse::<f64>()
    .map_err(|_| ErrorType::Invalid)?;

    let w1 = w.parse::<f64>()
    .map_err(|_| ErrorType::Invalid)?;

    Ok(Calc {height: h1, weight: w1})

}

fn calucate_bmi(s: Result<Calc, ErrorType>) -> f64 {
    if let Ok(v) = s {
        return v.weight / (v.height * v.height);
    }
    0.0
}

fn classify(bmi: f64) -> BodyType {
    if bmi < 18.0 {
        return BodyType::Slim;
    }

    if 18.0 <= bmi && bmi <= 24.0 {
        return BodyType::Standard;
    }

    else { return BodyType::Fat;}
}

fn main() {
    let h = "1.75";
    let w = "70";

    let m1 = check_input(h, w);
    let m2 = calucate_bmi(m1);
    let m3 = classify(m2);

    println!("{}, {:?}", m2, m3);
}