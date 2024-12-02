use crate::common;

pub fn result() {
    let mut contents = common::read_content_from_file("d2_p1__input.txt");
    let lines = contents.lines().collect::<Vec<&str>>();
    let mut numbers: Vec<i32> = Vec::new();
    let mut conditions: [bool; 3] = [false, false, false];
    let mut safe_reports = 0;

    // For each &str line ("<number> <number>"): split, parse as i32 and collects as a pair (Vec<i32>)
    for line in &lines {
         numbers = line
            .split(" ")
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        // 1° Law: the levels are either all increasing or all decreasing;
        let lone = law_one(&numbers);
        // 2° Law: any two adjacent levels differ by at least one and at most three (1<=diff=>3)
        let ltwo = law_two(&numbers);
        conditions = <[bool; 3]>::try_from([&lone[..], &ltwo[..]].concat().as_slice()).unwrap();

        safe_reports += if (conditions[0] || conditions[1]) && conditions[2] {
            1
        } else {
            0
        }
    }

    println!("D2:: Safe reports: {safe_reports}/{}", lines.len());
}

fn law_one(numbers: &Vec<i32>) -> [bool; 2] {
    let mut conditions: [bool; 2] = [false, false];

    conditions[0] = numbers.windows(2).all(|w| w[0] >= w[1]);
    conditions[1] = numbers.windows(2).all(|w| w[0] <= w[1]);

    conditions
}

fn law_two(numbers: &Vec<i32>) -> [bool; 1] {
    [numbers.windows(2).all(|w| {
        let difference = (w[0] - w[1]).abs();

        difference >= 1 && difference <= 3
    })]
}