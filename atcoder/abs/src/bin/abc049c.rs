use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("err");

    let vec: Vec<&str> = line.split_whitespace().collect();

    let a: u32 = vec[0].trim().parse().expect("err");
    let b: u32 = vec[1].trim().parse().expect("err");

    if a * b % 2 == 0 {
        println!("Even");
    } else {
        println!("Odd");
    }
}
