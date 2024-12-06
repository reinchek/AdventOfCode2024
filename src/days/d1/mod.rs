use crate::common;

fn part1(file_name: &str, sorted: bool) -> (Vec<i32>, Vec<i32>) {
    let mut vec_sx: Vec<i32> = Vec::new();
    let mut vec_dx: Vec<i32> = Vec::new();
    let mut contents = String::new();

    contents = common::read_content_from_file(file_name);

    let lines = contents.lines();
    for line in lines {
        let pair = line
            .split("   ")
            .into_iter()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        vec_sx.push(pair[0]);
        vec_dx.push(pair[1]);
    }

    if sorted {
        vec_sx.sort();
        vec_dx.sort();
    }

    (vec_sx, vec_dx)
}

fn part2 () {
    let (vec_sx, vec_dx) = part1("d1_p2__input.txt", false);
    let mut score = 0;

    for sx in vec_sx {
        let repetitions = vec_dx.iter().filter(|v| **v == sx).count();
        score += sx * repetitions as i32
    }

    println!("D1 (part 2)  :: {:?}", score);
}

pub fn result() {
    let input = part1("d1_p1__input.txt", true);
    let mut total_distance: i32 = 0;
    let mut counter = 0;

    for sx_value in input.0 {
        total_distance += (sx_value - &input.1[counter]).abs();
        counter += 1;
    }

    println!("D1 (part 1)  :: Total distance: {}", total_distance);

    part2();
}