fn main() {
    let hoge1 = [[true; 16384]; 14];
    let hoge2 = [[false; 16834]; 14];
    for i in 0..16384 {
        for j in 0..14 {
            println!("{}", hoge1[j][i]);
            println!("{}", hoge2[j][i]);
        }
    }
}
