fn f(x: i32) -> i32 {
    f(x)
}

fn g(x: f32) -> f32 {
    g(x)
}

fn h(x: i32) -> f32 {
    h(x)
}

fn i(x: f32) -> i32 {
    i(x)
}

fn main() {
    for n in 0..=10 {
        dbg!(f(n));
    }
    for r in 0..=10 {
        dbg!(g(r as f32));
    }
    for n in 0..=10 {
        dbg!(h(n));
    }
    for r in 0..=10 {
        dbg!(i(r as f32));
    }
}
