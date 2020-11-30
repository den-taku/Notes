fn id<T>(x: T) -> T {
    x
}

fn type_of<T>(_: T) -> String {
    std::any::type_name::<T>().to_string()
}

fn main() {
    println!("{}", type_of(|| {id}));
    println!("Hello, world!");
}
