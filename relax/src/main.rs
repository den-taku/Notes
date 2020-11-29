fn main() {
    let f = |x: f32| -> f32 { x.powf(8.0) + 9.0 };
    println!("{}", f(8.0));
}
