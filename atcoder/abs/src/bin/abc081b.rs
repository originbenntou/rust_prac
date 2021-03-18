#![allow(unused)]

use std::io::*;
use std::str::FromStr;

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

fn judge_rest(v: u32) -> Option<u32> {
    match v % 2 {
        0 => Some(v / 2),
        _ => None,
    }
}

fn judge_loop(v: Option<u32>) -> bool {
}

fn main() {
    let n = read::<u32>();
    let mut a: Vec<u32> = Vec::new();

    let mut ans: u32 = 0;

    for _ in 0..n {
        a.push(read::<u32>())
    }

    'outer: loop {
        // 'inner: for v in &mut a {
        //     if *v % 2 != 0 {
        //         break 'outer;
        //     }
        //     *v = *v / 2;
        // }

        'inner: for v in &mut a {
            match judge_rest(*v) {
                Some(s) => { *v = s; },
                None => { break 'outer; },
            }
        }

        ans += 1;
    }


    println!("{}", ans);
}
