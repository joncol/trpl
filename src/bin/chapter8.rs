use std::collections::HashMap;

fn main() {
    let (mean, median, mode) =
        mean_median_mode(vec![5, 9, 1, 4, 7, 2, 7, 3, 4, 7]);

    println!("Mean is: {}", mean);
    println!("Median is: {}", median);
    println!("Mode is: {}", mode);
}

fn mean_median_mode(v: Vec<i32>) -> (f32, f32, i32) {
    let mean = v.iter().sum::<i32>() as f32 / v.len() as f32;
    let mut sorted: Vec<i32> = v.to_vec();
    sorted.sort();
    let mid = sorted.len() / 2;
    let median =
        if sorted.len() % 2 == 0 {
            ((sorted[mid - 1] + sorted[mid]) as f32) / 2.0
        } else {
            sorted[mid] as f32
        };
    let mut map: HashMap<i32, i32> = HashMap::new();
    for x in v.iter() {
        let count = map.entry(*x).or_insert(0);
        *count += 1;
    }

    let mode = map
        .iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .map(|(k, _v)| k);

    (mean, median, *mode.unwrap())
}
