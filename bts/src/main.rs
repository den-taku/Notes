use std::collections::BTreeSet;

fn main() {
    let mut set = BTreeSet::new();
    for i in 0..5 {
        set.insert(i);
    }
    println!("{}", set.pop_last().unwrap());
    println!("Hello, world!");
}
