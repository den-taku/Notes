fn main() {
    let mut x: f64 = 1e8;
    for _ in 1..20 {
        println!("e={}", (1.0+1.0/x).powf(x));
        x += 1e8;
    }
}
