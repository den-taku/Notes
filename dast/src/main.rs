use std::fmt;
use std::ops::Add;

#[derive(Copy, Clone, Debug)]
struct Complex {
    re: f64,
    im: f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}+{}i", self.re, self.im)
    }
}

impl Add for Complex {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Complex {
            re: self.re + other.re,
            im: self.im + other.im,
        }
    }
}

fn main() {
    let a = Complex { re: 4.0, im: 5.0 };
    let b = Complex { re: 9.0, im: -9.0 };
    println!("{} + {} = {}", a, b, a + b);
    println!("{}", 3 != 0);
}
