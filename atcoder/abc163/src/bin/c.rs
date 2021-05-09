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
    let n = read::<usize>();
    let mut employees = (1..n).map(|_| read::<usize>()).collect::<Vec<usize>>();

    employees.insert(0, 0);

    let mut answer = Vec::with_capacity(n);
    for i in 1..=n {
        let count = employees.clone().into_iter().filter(|&x| i == x).count();
        answer.push(count);
    }

    for v in answer.iter() {
        println!("{}", v);
    }

    // println!("{}", answer.iter().map(|x| x.to_string()).collect::<Vec<String>>().join("\n"));
}
