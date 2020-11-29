fn tuple() -> (i32, i32, i32) {
    (3, 4, 5)
}

fn main() {
    let (a, b, c) = tuple();
    println!("{}, {}, {}", a, b, c);
}
