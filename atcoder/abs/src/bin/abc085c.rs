#![allow(unused)]

use std::io::*;
use std::str::FromStr;
use std::cmp::Reverse;

fn read<T: FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok().unwrap();
    s.trim().parse().ok().unwrap()
}

fn main() {
    let _ = read::<u32>();
    let mut vec = read::<String>().split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect::<Vec<u32>>();

    vec.sort_by_key(|&x| Reverse(x));

    let mut alice_score: u32 = 0;
    let mut bob_score: u32 = 0;
    for (i, v) in vec.iter().enumerate() {
        if i % 2 == 0 {
            alice_score += v;
        } else {
            bob_score += v;
        }
    }

    println!("{}", alice_score - bob_score);
}
