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

const BOIN: [char; 5] = ['a', 'i', 'u', 'e', 'o'];

fn main() {
    printer(&read::<char>());
}

fn printer(c: &char) {
    if BOIN.contains(c) {
        println!("vowel");
        return
    }
    println!("consonant");
}
