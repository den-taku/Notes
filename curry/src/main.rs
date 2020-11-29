fn curry(a: i32) -> Box<Fn(i32) -> i32> {
    Box::new(move |b| a + b)
}

fn main() {
    let a = 1;
    let b = 2;
    println!("{}", curry(a)(b));
}
