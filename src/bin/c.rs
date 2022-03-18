fn main() {
    proconio::input! {
        n: usize,
        w: i64,
        mut cheese: [(i64, i64); n],
    };

    cheese.sort_by_key(|&(a, _)| -a);

    let (ans, _) = cheese.into_iter()
                         .fold((0, w), |(count, remain), (deli, weight)| {
                             if weight <= remain {
                                 (count + deli * weight, remain - weight)
                             } else {
                                 (count + deli * remain, 0)
                             }
                         });

    println!("{}", ans);
}
