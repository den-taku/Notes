use rand::Rng;

enum Lab {
    Sato,
    Hashimoto,
    Nakamura,
}

fn main() {
    let mut rng = rand::thread_rng();

    if rng.gen() {
        println!("i32: {}, u32: {}", rng.gen::<i32>(), rng.gen::<u32>());
    }
    println!("f32: {}, f64: {}", rng.gen::<f32>(), rng.gen::<f64>());
    println!("f64: {}", f64::rand(rng));
}
