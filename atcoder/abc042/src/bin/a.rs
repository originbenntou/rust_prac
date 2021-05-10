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
    let mut phrase = (0..3).map(|_| read::<u32>()).collect::<Vec<u32>>();

    let mut response: &str = "";
    for v in [5, 7, 5].iter() {
        response = match phrase.iter().position(|x| x == v) {
            Some(n) => {
                phrase.remove(n);
                "YES"
            },
            None => "NO"
        };
        if response == "NO" { break }
    };

    println!("{}", response);
}
