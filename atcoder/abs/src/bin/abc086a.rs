#![allow(unused)]

use std::io::*;
use std::str::FromStr;

fn read<T: FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok().unwrap();
    s.trim().parse().ok().unwrap()
}

fn main() {
    let p = read::<String>().split_whitespace().filter_map(|x| x.parse::<u32>().ok()).product::<u32>();
    if p % 2 == 0 {
        println!("Even")
    } else {
        println!("Odd")
    }
}
