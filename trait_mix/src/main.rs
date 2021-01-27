trait A {
    fn hoge();
}

trait B : A {
    fn fuga();
}

trait C : B {
    fn foo();
}

trait D {
    fn bar();
}

trait E : A + D {
    fn var();
}

fn main() {
    println!("Hello, world!");
}
