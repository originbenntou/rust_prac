#![allow(unused)]

use std::io::*;
use std::str::FromStr;
use std::collections::HashSet;
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

    let unique = v.into_iter().collect::<HashSet<u32>>();

    println!("{:?}", unique.len());
}
