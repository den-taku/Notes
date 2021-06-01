fn main() {
    let a1 = [1, 5, 9];
    let a2 = [2, 6, 10];
    let a3 = [3, 7, 11];
    let a4 = [4, 8, 12];
    println!("{}", all(&a1, &a2, &a3, &a4));
}

fn all(a1: &[i64], a2: &[i64], a3: &[i64], a4: &[i64]) -> i64 {
    let mut ans = std::i64::MAX;
    for e1 in a1 {
        for e2 in a2 {
            for e3 in a3 {
                for e4 in a4 {
                    let est = cal_dis(*e1, *e2, *e3, *e4);
                    ans = std::cmp::min(est, ans);
                }
            }
        }
    }
    ans
}

fn _smallest(_a1: &[i64], _a2: &[i64], _a3: &[i64], _a4: &[i64]) -> i64 {
    let mut _ans = std::i64::MAX;
    unimplemented!()
}

#[test]
fn test_for_all() {
    let a1 = [1, 5, 9];
    let a2 = [2, 6, 10];
    let a3 = [3, 7, 11];
    let a4 = [4, 8, 12];
    assert_eq!(all(&a1, &a2, &a3, &a4), 10);
}

fn cal_dis(a1: i64, a2: i64, a3: i64, a4: i64) -> i64 {
    (a1 - a2).abs()
        + (a1 - a3).abs()
        + (a1 - a4).abs()
        + (a2 - a3).abs()
        + (a2 - a4).abs()
        + (a3 - a4).abs()
}
