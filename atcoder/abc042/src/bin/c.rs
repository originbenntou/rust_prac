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
    let mut n = read::<u32>();
    let k = read::<u32>();

    let dislikes = (0..k).map(|_| read::<u32>()).collect::<Vec<u32>>();

    'outer: loop {
        'inner: for (i, v) in dislikes.iter().enumerate() {
            if is_hate_appear(n, *v) {
                n += 1;
                break 'inner;
            }

            if i + 1 == dislikes.len() {
                break 'outer;
            }
        }
    };

    println!("{}", n);
}

fn is_hate_appear(price: u32, dislike: u32) -> bool {
    price.to_string().contains(&dislike.to_string())
}
