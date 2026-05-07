
use std::fmt;
trait Field {
    fn zero() -> Self;
    fn add(&self, other: &Self) -> Self;
    fn double(&self) -> Self;
}

struct Fp(u64);

impl fmt::Display for Fp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Fp({})", self.0)
    }
}

impl Field for Fp {
    fn zero() -> Self {
        Fp(0)
    }
    fn add(&self, other: &Self) -> Self {
        Fp(self.0 + other.0)
    }
    fn double(&self) -> Self {
        self.add(&self)
    }
}

// fn double<T: Field>(x: &T) -> T {
//     x.add(x)
// }

fn main() {
    let a = Fp(10);
    let b = Fp(32);
    let c = a.add(&b);
    let z = Fp::zero();
    let d = a.double();

    // println!("a = {}", a.0);
    // println!("b = {}", b.0);
    // println!("c = a + b = {}", c.0);
    // println!("zero = {}", z);

    println!("a = {}", a.0);
    println!("b = {}", b.0);
    println!("c = a + b = {}", c.0);
    println!("z = {}", z);
    println!("The d = {}", d.0);

}
