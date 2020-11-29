fn return_two() -> (i32, i32) {
    (3, 4)
}

fn receive_two(a: i32, b: i32) {
    println!("{} + {} = {}", a, b, a+b);
}

fn main() {
    // receive_two(return_two);
}
