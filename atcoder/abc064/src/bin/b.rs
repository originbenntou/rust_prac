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
    let coordinates = (0..n).map(|_| read::<u32>()).collect::<Vec<u32>>();

    let max = coordinates.iter().max().unwrap();
    let min = coordinates.iter().min().unwrap();

    println!("{}", max - min);
}
