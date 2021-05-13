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
    let w = read::<usize>();
    let a = read::<usize>();
    let b = read::<usize>();

    let mut squares: Vec<Vec<Option<u32>>> = vec![
        vec![Some(1); w]; h
    ];

    for i in 0..a {
        for j in 0..b {
            match squares.get_mut(h - i - 1) {
                Some(e) => {
                    e[j] = None;
                },
                None => {
                    println!("破滅");
                }
            }
        }
    }

    for i in 0..h {
        for j in 0..w {
            if i == 0 || j == 0 {
                continue;
            }

            if squares[i][j] == None {
                continue;
            }

            let mut up = squares[i - 1][j];
            if up == None {
                up = Some(0);
            }

            let mut left = squares[i][j - 1];
            if left == None {
                left = Some(0);
            }

            let sum = up.unwrap() + left.unwrap();

            squares[i][j] = Some(sum);
        }
    }

    println!("{:?}", squares);
}
