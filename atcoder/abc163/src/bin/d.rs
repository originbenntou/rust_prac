use std::io::*;
use std::str::FromStr;
use std::collections::HashSet;
use once_cell::sync::OnceCell;

static BASE_NUMBERS_LEN: OnceCell<usize> = OnceCell::new();

struct CalculateStore {
    temp: Vec<u32>,
    result: Vec<u32>,
}

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
    let n = read::<u32>();
    let k = read::<u32>();

    let numbers = (0..=n).into_iter().map(|x| x + 10_u32.pow(7)).collect::<Vec<u32>>();

    BASE_NUMBERS_LEN.set(numbers.len());

    let mut calculate_store = CalculateStore {
        temp: Vec::with_capacity((k + 1) as usize),
        result: Vec::with_capacity((k + 1) as usize)
    };

    for i in k..=n+1 {
        recursive(numbers.clone(), &mut calculate_store,(i - 1) as usize);
    }

    let uniq: HashSet<u32> = calculate_store.result.into_iter().collect();

    println!("{}", uniq.len());
}

fn recursive(numbers: Vec<u32>, store: &mut CalculateStore, depth: usize) {
    let now_len = numbers.len();
    let diff_len = BASE_NUMBERS_LEN.get() - now_len;

    for i in 0..now_len {
        // 数列から数値を取得し格納、最下層まで辿り着いたら格納した数値を合計
        store.temp.push(numbers[i]);

        if diff_len < depth {
            // 最下層でなければ数列から数値を削除し、次の階層へ渡す
            let mut new_numbers = numbers.clone();
            new_numbers.remove(i);
            recursive(new_numbers, store, depth);
        } else {
            // 最下層なら格納した数値を合計、結果コレクションへ格納
            store.result.push(store.temp.iter().sum());
            store.temp.pop(); // 次の計算に最後の数値は不要なので削除
        }
    }

    // 最下層すべての計算が終わったら一階層戻り、一階層前で格納した最後の数値は不要のため削除
    store.temp.pop();
}
