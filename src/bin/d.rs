use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
        k: i64,
    };

    let mut state = 0;
    let mut starts = Vec::<usize>::new();

    for (index, &c) in s.iter().enumerate() {
        if c == '.' {
            state = 0;
        } else {
            if state == 0 {
                starts.push(index);
            }

            state = 1;
        }
    }

    let ans = starts.into_iter()
                    .map(|start| count(&s, k, start))
                    .max()
                    .unwrap();

    println!("{}", ans);
}

fn count(s: &Vec<char>, mut k: i64, start: usize) -> u64 {
    let mut count = 0;
    let mut iter = s.iter().skip(start);

    while let Some(&c) = iter.next() {
        if c == '.' {
            if 0 < k {
                k -= 1;
                count += 1;
            } else {
                break;
            }
        } else {
            count += 1;
        }
    }

    return count;
}
