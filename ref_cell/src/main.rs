use std::cell::RefCell;

fn ref_refcell(refed: i32) {
    println!("Ok! : {}", refed);
}

fn main() {
    let refed = RefCell::new(56i32);
    ref_refcell(refed);
}
