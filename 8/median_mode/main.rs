use std::collections::HashMap;

fn median_and_mode(numbers: &mut Vec<i32>) -> (f32, i32) {
    numbers.sort_unstable();
    let median = if numbers.len() % 2 == 0 {
        let mid1 = numbers[numbers.len() / 2 - 1];
        let mid2 = numbers[numbers.len() / 2];
        (mid1 as f32 + mid2 as f32) / 2.0
    } else {
        numbers[numbers.len() / 2] as f32
    };

    let mut occurrences = HashMap::new();
    for &number in numbers.iter() {
        *occurrences.entry(number).or_insert(0) += 1;
    }

    let mode = occurrences
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(val, _)| val)
        .expect("Cannot compute mode of an empty list");

    (median, mode)
}

fn main() {
    let mut numbers = vec![1, 3, 3, 6, 7, 8, 9, 9, 9];
    let (median, mode) = median_and_mode(&mut numbers);
    println!("Median: {}, Mode: {}", median, mode);
}
