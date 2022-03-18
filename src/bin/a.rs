fn main() {
    proconio::input! {
        s1: String,
        s2: String,
    };

    let check = |a: &String, b: &String| {
        a == ".#" && b == "#."
    };

    let ans = check(&s1, &s2) || check(&s2, &s1);

    println!("{}", if ans {"No"} else {"Yes"});
}
