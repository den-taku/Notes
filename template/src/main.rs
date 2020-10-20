use num_traits::{Float, FromPrimitive};

fn f<T>(input: T) -> T 
where
    T: Float + FromPrimitive
{
    input / T::from_f32(4.0).unwrap()
}


fn main() {
    println!("Hello, world!");
}
