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

struct Color {
    r: u32,
    g: u32,
    b: u32,
}

fn main() {
    let color = Color {
        r: read::<u32>(),
        g: read::<u32>(),
        b: read::<u32>(),
    };

    let number = color.r.to_string() + &color.g.to_string() + &color.b.to_string();
    if number.parse::<u32>().unwrap() % 4 == 0 {
        println!("YES");
        return;
    }

    println!("NO");
}
