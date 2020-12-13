fn main() {
    let mut v = Vec::new();
    for i in 0..10 {
        for j in 0..10 {
            v.push(i * j);
        }
    }
    for e in v.iter() {
        println!("{}", e);
    }
}
