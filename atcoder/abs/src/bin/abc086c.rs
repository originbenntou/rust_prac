use std::str::FromStr;

fn read<T: FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok().unwrap();
    s.trim().parse().ok().unwrap()
}

fn main() {
    let n = read::<u32>();

    let mut xy_init = [0; 2];

    for _ in 0..n {
        let input = read::<String>().split_whitespace().filter_map(|x| x.parse::<i32>().ok()).collect::<Vec<i32>>();

        if xy_init == [input[1], input[2]] {
            println!("No");
            return;
        }

        let mut xy_list: Vec<[i32; 2]> = Vec::new();

        for j in 1..=input[0] {
            for k in 0..j + 1 {
                xy_list.push([j - k - xy_init[0], k - xy_init[1]]);
            }
        }

        match xy_list.iter().find(|&&x| x == [input[1] - xy_init[0], input[2] - xy_init[1]]) {
            None => {
                println!("No");
                return;
            },
            Some(_) => {
                xy_init = [input[1], input[2]];
            }
        }
    }

    println!("Yes");
}
