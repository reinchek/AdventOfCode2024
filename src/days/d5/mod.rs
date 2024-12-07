use std::collections::BTreeMap;
use crate::common;

pub fn result() {
    let mut reordered_total = 0;
    let mut total = 0;
    let input_data = common::read_content_from_file("d5_p1__input.txt");
    // Get page rules
    let rules = input_data
        .lines()
        .filter(|line| line.contains("|"))
        .map(|line| {
            line.split("|").map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let pages_to_check = input_data
        .lines()
        .filter(|line| line.contains(","))
        .map(|line| line.split(",").map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>())
        .collect::<Vec<Vec<i32>>>();

    for pages in pages_to_check.iter() {
        let mut ordered_hmap = BTreeMap::<i32, i32>::new();

        for page in pages.iter() {
            let times = rules.iter().filter(|rule| {
                rule[0] == *page && pages.contains(&rule[1])
            }).count();
            ordered_hmap.insert(times as i32, *page);
        }
        let ordered_hmap = ordered_hmap.iter().map(|m| { *m.1 }).rev().collect::<Vec<i32>>().to_vec();
        // Check if is correctly ordered
        if compare(&pages, &ordered_hmap) {
            // Get middle page num:
            total += pages[(pages.len() - 1) / 2];
        } else {
            reordered_total += ordered_hmap[(ordered_hmap.len() - 1) / 2];
        }
    }

    println!("D5 (part 1)  :: Sum of all middle ordered pages: {}", total);
    println!("D5 (part 2)  :: Sum of all middle fixed (re)ordered pages: {}", reordered_total);
}

fn compare<T: PartialEq>(a: &Vec<T>, b: &Vec<T>) -> bool {
    let matching = a.iter().zip(b.iter()).filter(|&(a, b)| a == b).count();
    matching == a.len() && matching == b.len()
}
