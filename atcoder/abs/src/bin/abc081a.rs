use proconio::input;
use proconio::marker::{Chars};

fn main() {
    input! {
        n: Chars,
    }

    let mut count: i32 = 0;
    for i in n {
        if (i as i32 - 48) == 1 {
            count += 1;
        }
    }

    println!("{}", count);
}
