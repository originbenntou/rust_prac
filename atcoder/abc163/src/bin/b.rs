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

fn main() {
    let n = read::<u32>();
    let m = read::<u32>();

    let s = (0..m).map(|_| read::<u32>()).collect::<Vec<u32>>().into_iter().sum::<u32>();

    let answer = n as i32 - s as i32;

    if answer >= 0 {
        println!("{}", answer);
    } else {
        println!("{}", -1);
    };
}
