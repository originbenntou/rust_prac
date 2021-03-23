#![allow(unused)]

use std::io::*;
use std::str::FromStr;

fn read<T: FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok().unwrap();
    s.trim().parse().ok().unwrap()
}

fn main() {
    let s = read::<String>()
        .split_whitespace().filter_map(|x| x.parse().ok())
        .collect::<Vec<u32>>();

    println!("{}", (1..s[0] + 1)
        .filter(|x| {
            let digit_sum = x.to_string().chars().map(|x| x as u32 - 48).sum::<u32>();
            digit_sum >= s[1] && digit_sum <= s[2]
        })
        .fold(0, |mut sum, x| sum + x));
}
