use std::cmp::{max, min};
use crate::common;


pub fn result() {
    let mut contents = common::read_content_from_file("d2_p1__input.txt");
    let lines = contents.lines().collect::<Vec<&str>>();
    let mut numbers: Vec<i32> = Vec::new();
    let mut safe_reports = 0;

    // For each &str line ("<number> <number>"): split, parse as i32 and collects as a pair (Vec<i32>)
    for line in &lines {
        numbers = line
            .split(" ")
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let mut sub_numbers: Vec<Vec<i32>> = Vec::new();
        for i in 0..numbers.len() {
            let mut sub: Vec<i32> = Vec::from(&numbers[..i]);
            sub.append(&mut Vec::from(&numbers[i + 1..]));
            sub_numbers.push(sub);
        };

        if is_valid(&numbers) || sub_numbers.iter().any(|vec| is_valid(&vec)) {
            safe_reports += 1
        }
    }

    println!("Safe reports: {}", safe_reports);
}

fn is_valid(numbers: &Vec<i32>) -> bool {
    let mut numbers_copy = numbers.clone();
    let conditions = [
        numbers_copy.windows(2).all(|w| w[0] > w[1]), // All DESC
        numbers_copy.windows(2).all(|w| w[0] < w[1])  // All ASC
    ];

    let mut valid = conditions[0] || conditions[1];

    if valid {
        numbers_copy.sort();
        for window in numbers_copy.windows(2) {
            if let &[a, b] = window {
                if b - a > 3 || b - a < 1 {
                    valid = false;
                    break;
                }
            }
        }
    }

    valid
}