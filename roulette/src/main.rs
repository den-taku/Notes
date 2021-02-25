use rand::Rng;

enum Lab {
    Sato,
    Hashimoto,
    Nakamura,
    Doi,
    Kurohashi,
    Nishino,
    Yamamoto,
}

fn main() {
    let mut rng = rand::thread_rng();
    rng.gen();
    println!("Your Lab is ...");
}
