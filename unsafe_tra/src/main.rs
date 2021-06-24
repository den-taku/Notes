fn main() {
    println!("Unsafe!");
    let x = 3;
    println!("*x : {:?}", &x as *const i32);
}
