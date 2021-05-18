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
    let h = read::<usize>();
    let _ = read::<usize>();

    let mut scaled = Vec::with_capacity(h);
    for _ in 0..h {
        let row = read::<String>();
        scaled.push(row.clone());
        scaled.push(row.clone());
    }

    println!("{}", scaled.join("\n"));
}
