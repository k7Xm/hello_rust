struct PP (f64,f64,);
fn main() {
    let p = PP(1.0, 2.0);

    println!("first coordinate is \"{}\"; Second coordinate is \"{}\".", p.0, p.1);
}