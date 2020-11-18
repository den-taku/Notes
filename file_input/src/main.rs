use std::fs::File;
use std::io::copy;

fn main() {
    let mut file_txt = File::open("text.txt").expect("txt error");
    let mut file_asm = File::open("text.asm").expect("asm error");

    let mut vec_txt = Vec::new();
    let mut vec_asm = Vec::new();

    let _ = copy(&mut file_txt, &mut vec_txt);
    let _ = copy(&mut file_asm, &mut vec_asm);

    println!("{}", String::from_utf8(vec_txt).unwrap());
    println!("{}", String::from_utf8(vec_asm).unwrap());
}
