fn something(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        let mut r = a;
        while r >= b {
            r = r - b
        }
        something(b, r)
    }
}

fn main() {
    println!("{}", something(0, -3));
}
