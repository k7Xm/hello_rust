use std::fmt;

trait Field {
    fn zero() -> Self;
    fn add(&self, other: &Self) -> Self;
}

struct Fp(u64);

impl Field for Fp {
    fn zero() -> Self {
        Fp(0)
    }
    fn add(&self, other: &Self) -> Self {
        Fp(self.0 + other.0)
    }
}

impl fmt::Display for Fp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Fp({})", self.0)
    }
}

fn double<T: Field>(x: &T) -> T {
    x.add(x)
}

fn show_and_double<T>(val: &T) -> T
where
    T: Field + fmt::Display,
{
    println!("Inital Value: {}", val);
    let result = val.add(val);
    println!("Doubled value: {}", result);
    result
}

fn main() {
    let a = Fp(7);
    let b = double(&a);
    let c = show_and_double(&a);
    println!("b = {}, c = {}", b, c);
}
