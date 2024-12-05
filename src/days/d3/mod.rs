use crate::common;
use regex::Regex;

// Day 3: Corrupted memory.
//        Corrupted example data: xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
//        You have to consider only mul(<n>, <n>) instructions and then sum all together.
//        For this challenge we use "regex" crate. Rust doesn't have std lib for regular expressions.
//        Part 2: multiply only enabled (by "do()" prev instruction) "mul()" and ignore disabled (by "don't()")
//                "mul()" instructions.
pub fn result() {
    let content = common::read_content_from_file("d3_p1__input.txt");
    let mut total = 0;
    let mut enabled = true;

    // Regex: mul\(\d+,\d+\)|do\(\)|don't\(\)
    let re = Regex::new(r"mul\(\d+,\d+\)|do\(\)|don't\(\)").unwrap();
    let re_mul = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    re.captures_iter(content.as_str()).for_each(|v| {
        let text: (&str, [&str; 0]) = v.extract();
        if text.0 == "do()" {
            enabled = true;
        } else if text.0 == "don't()" {
            enabled = false;
        } else {
            if enabled {
                re_mul.captures(text.0).iter().for_each(|v| {
                    let v: (&str, [&str; 2]) = v.extract();
                    let mul = v.1.iter().map(|x| x.parse::<i32>().unwrap()).product::<i32>();
                    total += mul;
                });
            }
        }
    });

    println!("Total: {}", total);
}