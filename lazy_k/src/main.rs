fn I<T>(x: T) -> T {
    x
}

fn K<T, U>(x: T, y: U) -> U {
    x
}

fn S<T, U, V>(x: T, y: U, z: V) {
    // (x y) (x z)
}

fn main() {
    println!("Hello, world!");
}
