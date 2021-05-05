
#![allow(unused)]

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    println!("{:p}", s.as_bytes());
    println!("{:p}", s.as_bytes());
    println!("{:p}", bytes);

    for (i, item) in bytes.iter().enumerate() {
        println!("{}", *item);
        if item == &b' ' {
            return i;
        }
    }
    s.len()
}

fn main() {
    let s = String::from("hello world");
    println!("{:p}", &s);

    println!("{}", first_word(&s));
    println!("{number:>width$}", number=1, width=6);
}
