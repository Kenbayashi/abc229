use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        a: Chars,
        b: Chars,
    }

    let a = a.into_iter()
             .map(|c| c.to_string().parse::<u8>().ok().unwrap())
             .collect::<Vec<u8>>();

    let b = b.into_iter()
             .map(|c| c.to_string().parse::<u8>().ok().unwrap())
             .collect::<Vec<u8>>();

    let ans = a.into_iter()
               .rev()
               .zip(b.into_iter().rev())
               .map(|(left, right)| left + right)
               .all(|num| num < 10);

    println!("{}", if ans {"Easy"} else {"Hard"});
}
