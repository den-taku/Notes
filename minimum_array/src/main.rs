use std::cmp::min;

fn main() {
    let a1 = [2, 7, 9];
    let a2 = [3, 5, 10];
    let a3 = [4, 11];
    let a4 = [6, 8, 12];
    println!("{}", all(&a1, &a2, &a3, &a4));
    println!("{}", smallest(&a1, &a2, &a3, &a4));
    let a1 = [2, 7, 9, 13, 24, 56, 64, 100, 121, 123];
    let a2 = [3, 5, 10, 14, 28, 73, 74, 120, 150];
    let a3 = [4, 11, 20, 90, 200];
    let a4 = [6, 8, 12, 130, 131, 132, 133, 134, 135, 190];
    println!("{}", all(&a1, &a2, &a3, &a4));
    println!("{}", smallest(&a1, &a2, &a3, &a4));
}

fn smallest(a1: &[i64], a2: &[i64], a3: &[i64], a4: &[i64]) -> i64 {
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;
    let mut l = 0;
    let mut ans = std::i64::MAX;
    while i < a1.len() && j < a2.len() && k < a3.len() && l < a4.len() {
        let est = cal_dis(a1[i], a2[j], a3[k], a4[l]);
        ans = min(ans, est);
        let min = min(a1[i], min(a2[j], min(a3[k], a4[l])));
        if a1[i] == min {
            i += 1;
        } else if a2[j] == min {
            j += 1;
        } else if a3[k] == min {
            k += 1;
        } else {
            l += 1;
        }
    }
    ans
}

#[test]
fn test_for_smallest() {
    let a1 = [1, 5, 9];
    let a2 = [2, 6, 10];
    let a3 = [3, 7, 11];
    let a4 = [4, 8, 12];
    assert_eq!(smallest(&a1, &a2, &a3, &a4), 10);
}

#[test]
fn test_for_smallest2() {
    let a1 = [2, 7, 9];
    let a2 = [3, 5, 10];
    let a3 = [4, 11];
    let a4 = [6, 8, 12];
    assert_eq!(smallest(&a1, &a2, &a3, &a4), 10);
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

#[test]
fn test_for_all() {
    let a1 = [1, 5, 9];
    let a2 = [2, 6, 10];
    let a3 = [3, 7, 11];
    let a4 = [4, 8, 12];
    assert_eq!(all(&a1, &a2, &a3, &a4), 10);
}

#[test]
fn test_for_all2() {
    let a1 = [2, 7, 9];
    let a2 = [3, 5, 10];
    let a3 = [4, 11];
    let a4 = [6, 8, 12];
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

#[test]
fn test_for_all_smallest() {
    let a1 = [1, 5, 9];
    let a2 = [2, 6, 10];
    let a3 = [3, 7, 11];
    let a4 = [4, 8, 12];
    assert_eq!(all(&a1, &a2, &a3, &a4), smallest(&a1, &a2, &a3, &a4));
}

#[test]
fn test_for_all_smallest2() {
    let a1 = [2, 7, 9, 13, 24, 56, 64, 100, 121, 123];
    let a2 = [3, 5, 10, 14, 28, 73, 74, 120, 150];
    let a3 = [4, 11, 20, 90, 200];
    let a4 = [6, 8, 12, 130, 131, 132, 133, 134, 135, 190];
    assert_eq!(all(&a1, &a2, &a3, &a4), smallest(&a1, &a2, &a3, &a4));
}
