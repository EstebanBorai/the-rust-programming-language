//! Given a list of integers, use a vector and return the mean (the average value),
//! median (when sorted, the value in the middle position), and mode (the value that
//! occurs most often; a hash map will be helpful here) of the list.

use std::collections::hash_map::HashMap;
use utils::input;

fn main() {
    let mut mean = 0;
    let mut mode: (i32, i32) = (0, 0);
    let ref mut occurences: HashMap<i32, u8> = HashMap::new();
    let integers = input(Some("Enter integers separated by a space: "));
    let mut integers = integers
        .trim()
        .split(" ")
        .map(|v| v.parse::<i32>().expect(&format!(
            "Unexpected value {}, expected a valid interger",
            v
        )))
        .collect::<Vec<i32>>();

    integers.sort();

    integers.iter().for_each(|int| {
        if let Some(count) = occurences.get(&int) {
            occurences.insert(*int, *count + 1);
        } else {
            occurences.insert(*int, 1);
        }

        mean += int;
    });

    for (key, value) in occurences.iter() {
        if value > &(mode.1 as u8) {
            mode = (*key, *value as i32);
        }
    }

    println!("mean: {}", mean / integers.len() as i32);
    println!("median: {}", integers[integers.len() / 2]);
    println!("mode: {}", mode.0);
}
