const MOD: i64 = 1_000_000_007;

fn main() {
    let mut v = Vec::with_capacity(100_000);
    for i in 0..100_000 {
        v.push(i * i + 4 * i);
    }
    println!("{}", {
        let mut sum = 0;
        unsafe {
        for i in 0..100_000 {
            sum += *v.get_unchecked(i);
            sum %= MOD;
        }
        }
        sum
    });
}
