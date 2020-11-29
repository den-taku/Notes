struct Base<T> {
    n: usize,
    m: usize,
    array: Vec<T>,
}

impl<T> Base<T> {
    fn is_square(&self) -> bool {
        self.n == self.m
    }
}

struct Inherence<T> {
    n: usize,
    m: usize,
    array: Vec<T>,
    lu: bool
}


fn main() {
    println!("Hello, world!");
}
