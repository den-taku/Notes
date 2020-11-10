fn main() {
    let hoge1 = [true; 16384];
    let hoge2 = [false; 16834];
    for i in 0..16384 {
        println!("{}", hoge1[i]);
        println!("{}", hoge2[i]);
    }
}
