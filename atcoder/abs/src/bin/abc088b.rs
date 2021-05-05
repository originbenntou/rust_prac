use std::io::*;
use std::str::FromStr;
use std::cmp::Reverse;

fn read<T: FromStr>() -> T {
    let stdin = stdin();
    let stdin = stdin.lock();
    let token: String = stdin
        .bytes()
        .map(|c| c.expect("failed to read char") as char)
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
        .collect();
    token.parse().ok().expect("failed to parse token")
}

fn main() {
    let n = read::<u32>();
    let mut v = (0..n).map(|_| read::<u32>()).collect::<Vec<u32>>();

    v.sort_by_key(|&x| Reverse(x));

    let (mut alice_score, mut bob_score) = (0_u32, 0_u32);
    for (i, v) in v.iter().enumerate() {
        if i % 2 == 0 {
            alice_score += v;
        } else {
            bob_score += v;
        }
    }
    println!("{}", alice_score - bob_score);
}
