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

struct Store {
    pattern_result: Vec<String>,
    tmp: Vec<String>
}

impl<'a> Store {
    const LIST: [&'a str; 4] = [
        "eraser",
        "dreamer",
        "erase",
        "dream",
    ];

    pub fn generate_all_pattern(&mut self, depth: usize) {
        if self.tmp.len() > depth {
            self.pattern_result.push(self.tmp.join(""));
            self.tmp.pop();
            return;
        }

        for v in Self::LIST.iter() {
            self.tmp.push((*v.clone()).parse().unwrap());
            Self::generate_all_pattern(self, depth);
        }

        self.tmp.pop();
    }
}

fn main() {
    let mut s = Store{
        pattern_result: Vec::new(),
        tmp: Vec::new()
    };

    for i in 0..Store::LIST.len() {
        s.generate_all_pattern(i);
    }

    let input = read::<String>();

    println!("{:?}", s.pattern_result);

    for v in s.pattern_result.iter() {
        if v == &input {
            println!("YES");
            return;
        }
    }

    println!("NO");
}
