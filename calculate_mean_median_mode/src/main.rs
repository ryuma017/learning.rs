use std::collections::HashMap;

fn main() {
    let mut numbers = vec![1, 3, 1, 2, 3, 1, 3, 3, 2];

    let mean = {
        let sum: f64 = numbers.iter().map(|&val| val as f64).sum();
        let length: f64 = numbers.len() as f64;

        sum / length
    };

    #[allow(clippy::stable_sort_primitive)]
    let median = {
        numbers.sort(); // try: `v.sort_unstable()`ってclippyが言ってた。等しい要素の順序が保証されないけど高速らしい。
        let mid = numbers.len() / 2;
        if numbers.len() % 2 == 0 {
            (numbers[mid - 1] + numbers[mid]) / 2
        } else {
            numbers[mid]
        }
    };

    let mode: Vec<i32> = {
        let mut map = HashMap::new();
        for n in &numbers {
            let count = map.entry(n).or_insert(0);
            *count += 1;
        }

        let max_value = map.values().cloned().max().unwrap();

        map.into_iter()
            .filter(|&(_, v)| v == max_value)
            .map(|(&k, _)| k)
            .collect()
    };

    println!("{:?}", numbers);
    println!("mean: {}", mean);
    println!("median: {}", median);
    println!("mode: {}", mode[0]);
}
