use std::fs::File;

fn main() {
    let file = File::create("file.txt");
    println!("{:?}", file.unwrap());
    // println!("{:?}", file);
}
