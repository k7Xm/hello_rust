use std::fmt;
trait Field {
    fn zero() -> Self;
    fn add(&self, other: &Self) -> Self;
    fn mul(&self, other: &Self) -> Self;
}

struct Fp(u64);

impl Field for Fp {
    fn zero() -> Self {
        Fp(0)
    }

    fn add(&self, other: &Self) -> Self {
        Fp(self.0 + other.0)
    }

    fn mul(&self, other: &Self) -> Self {
        Fp(self.0 * other.0)
    }
}

//Display trait???
impl fmt::Display for Fp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Fp({})", self.0)
    }
}

fn square<T>(x: &T) -> T
where
    T: Field + fmt::Display,
{
    x.mul(x)
}

fn main() {
    let a = Fp(7);
    let b = Fp(3);
    let c = a.add(&b);
    let s = square(&a);
    let z = Fp::zero();
    let m = a.mul(&b);

    println!("a = {}", a);
    println!("b = {}", b);
    println!("a + b = {}", c);
    println!("a^2 = {}", s);
    println!("zero = {}", z);
    println!("a * b = {}", m);
}
