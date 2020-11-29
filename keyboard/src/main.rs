use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    for line_result in stdin.lock().lines() {
        let line = line_result.expect("err");
        println!("{:?}", line.as_bytes());
    }
    let input = "K";
    match input {
        "A" | "B" | "C" | "D" | "E" | "F" | "K" => println!("OK"),
         _ => ()
    }
}
