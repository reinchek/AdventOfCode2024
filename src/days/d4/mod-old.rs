use crate::common;
use std::fmt::Display;

// Day 4: Search "XMAS" occurrences.
//        Like "Connect 4" game.
const X_WORDS: [&str; 2] = ["XMAS", "SAMX"];
const XWORD_LEN: usize = X_WORDS[0].len() - 1;

#[derive(Debug, Clone, Copy)]
struct Letter {
    letter: char,
    used: bool
}


#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
    DiagonalUpLeft,
    DiagonalUpRight,
    DiagonalDownLeft,
    DiagonalDownRight
}

type XmasLetters = Vec<Letter>;
type XmasMatrix = Vec<XmasLetters>;

trait XmasTrait {
    fn check_direction(&self, curr_row: usize, curr_col: usize, direction: Direction) -> bool;
    fn matrix_string(&self, col_start: usize, col_end: usize, row_start: usize, row_end: usize) -> String;
    fn mark_as_used(&mut self, word: &str, col_start: usize, col_end: usize, row_start: usize, row_end: usize, total: &mut i32);
    fn format(&self) -> Vec<String>;
}
trait XmasTraitLetters {
    fn to_string(&self) -> String;
    fn to_sized_string(&self, start: usize, end: usize) -> String;
}
impl XmasTraitLetters for XmasLetters {
    fn to_string(&self) -> String {
        self.iter().map(|letter| letter.letter).collect::<String>()
    }
    fn to_sized_string(&self, start: usize, end: usize) -> String {
        if start > end {
            self.iter().map(|letter| letter.letter).collect::<Vec<char>>()[end..=start].iter().rev().collect::<String>()
        } else {
            self.iter().map(|letter| letter.letter).collect::<Vec<char>>()[start..=(end-1)].iter().collect::<String>()
        }
    }
}

impl XmasTrait for XmasMatrix {
    fn check_direction(&self, curr_row: usize, curr_col: usize, direction: Direction) -> bool {
        let col_len: usize = self[0].len();
        let row_len: usize = self.len();

        match direction {
            Direction::Left => curr_col.checked_sub(XWORD_LEN).is_some(),
            Direction::Right => curr_col + XWORD_LEN <= col_len,
            Direction::Up => curr_row.checked_sub(XWORD_LEN).is_some(),
            Direction::Down => { curr_row + XWORD_LEN <= row_len },
            Direction::DiagonalUpLeft => curr_col.checked_sub(XWORD_LEN).is_some() && curr_row.checked_sub(XWORD_LEN).is_some(),
            Direction::DiagonalUpRight => curr_col + XWORD_LEN <= col_len && curr_row.checked_sub(XWORD_LEN).is_some(),
            Direction::DiagonalDownLeft => curr_col.checked_sub(XWORD_LEN).is_some() && curr_row + XWORD_LEN <= row_len,
            Direction::DiagonalDownRight => curr_col + XWORD_LEN >= col_len && curr_row + XWORD_LEN <= row_len
        }
    }

    fn   matrix_string(&self, col_start: usize, col_end: usize, row_start: usize, row_end: usize) -> String {
        let mut rows = Vec::new();

        let mut counter: i32 = col_start as i32;
        let dir = if col_start > col_end { -1 } else { 1 };

        if row_start > row_end {
            rows = self[row_end..=row_start].iter().rev().collect();
        } else {
            rows = self[row_start..=(row_end-1)].iter().collect();
        };


        let extracted_word = rows.iter().map(|letter| {
            let len = letter.len();
            let letter = letter[counter as usize].letter;
            if col_start != col_end {
                counter += if counter < (len-1) as i32 { dir } else { 0 };
            }
            letter
        }).collect::<String>();

        extracted_word
    }

    fn mark_as_used(&mut self, word: &str, col_start: usize, col_end: usize, row_start: usize, row_end: usize, total: &mut i32) {
        println!("fn mark_as_used() called. total => {total}");
        if X_WORDS.contains(&word) {
            let (mut row_start, row_end) = if row_start > row_end { (row_end, row_start) } else { (row_start, row_end) };
            let (mut col_start, col_end) = if col_start > col_end { (col_end, col_start) } else { (col_start, col_end) };

            if row_start != row_end {
                if col_start == col_end {
                    while row_start <= row_end {
                        self[row_start][col_start].used = true;
                        row_start += 1;
                    }
                } else {
                    while row_start < row_end && col_start <= col_end {
                        self[row_start][col_start].used = true;
                        row_start += 1;
                        col_start += 1;
                    }
                }
            } else {
                for row in row_start..=row_end {
                    for col in col_start..=(col_end) {
                        self[row][col].used = true;
                    }
                }
            }

            *total += 1;
        }
    }

    fn format(&self) -> Vec<String> {
        self.iter().map(|letters| {
            letters.iter().map(|letter| if letter.used { letter.letter } else { '.' } ).collect::<String>()
        }).collect::<Vec<String>>()
    }
}

pub fn result () {
    let mut total = 0;
    // let input_data = common::read_content_from_file("d4_p1__input.txt");
    let input_data = common::read_content_from_file("d4_p1__demo.txt");
    let mut xmatrix: XmasMatrix;

    xmatrix = input_data
        .lines()
        .map(|l| l.bytes().map(|c| Letter {
            letter: c as char,
            used: false
        }).collect::<Vec<Letter>>())
        .collect::<XmasMatrix>();

    // Loop through all matrix's chars.
    for row in xmatrix.clone().iter().enumerate() {
        for letter in row.1.iter().enumerate() {

            let curr_row = row.0;
            let curr_col = letter.0;

            let mut extracted_word: String = String::new();

            if xmatrix.check_direction(curr_row, curr_col, Direction::Left) {
                extracted_word = row.1.to_sized_string(curr_col, curr_col-XWORD_LEN);
                xmatrix.mark_as_used(&extracted_word, curr_col, curr_col-XWORD_LEN, curr_row, curr_row, &mut total);
            }
            if xmatrix.check_direction(curr_row, curr_col, Direction::Right) {
                extracted_word = row.1.to_sized_string(curr_col, curr_col+XWORD_LEN);
                xmatrix.mark_as_used(&extracted_word, curr_col, curr_col+XWORD_LEN, curr_row, curr_row, &mut total);
            }
            if xmatrix.check_direction(curr_row, curr_col, Direction::Up) {
                extracted_word = xmatrix.matrix_string(curr_col, curr_col, curr_row, curr_row-XWORD_LEN);
                xmatrix.mark_as_used(&extracted_word, curr_col, curr_col, curr_row, curr_row-XWORD_LEN, &mut total);
            }
            if xmatrix.check_direction(curr_row, curr_col, Direction::Down) {
                extracted_word = xmatrix.matrix_string(curr_col, curr_col, curr_row, curr_row+XWORD_LEN);
                xmatrix.mark_as_used(&extracted_word, curr_col, curr_col, curr_row, curr_row+XWORD_LEN, &mut total);
            }
            if xmatrix.check_direction(curr_row, curr_col, Direction::DiagonalDownLeft) {
                extracted_word = xmatrix.matrix_string(curr_col, curr_col-XWORD_LEN, curr_row, curr_row+XWORD_LEN);
                xmatrix.mark_as_used(&extracted_word, curr_col, curr_col-XWORD_LEN, curr_row, curr_row+XWORD_LEN, &mut total);
            }
            if xmatrix.check_direction(curr_row, curr_col, Direction::DiagonalDownRight) {
                extracted_word = xmatrix.matrix_string(curr_col, curr_col+XWORD_LEN, curr_row, curr_row+XWORD_LEN);
                xmatrix.mark_as_used(&extracted_word, curr_col, curr_col+XWORD_LEN, curr_row, curr_row+XWORD_LEN, &mut total);
            }
            if xmatrix.check_direction(curr_row, curr_col, Direction::DiagonalUpLeft) {
                extracted_word = xmatrix.matrix_string(curr_col, curr_col-XWORD_LEN, curr_row, curr_row-XWORD_LEN);
                xmatrix.mark_as_used(&extracted_word, curr_col, curr_col-XWORD_LEN, curr_row, curr_row-XWORD_LEN, &mut total);
            }
            if xmatrix.check_direction(curr_row, curr_col, Direction::DiagonalUpRight) {
                extracted_word = xmatrix.matrix_string(curr_col, curr_col+XWORD_LEN, curr_row, curr_row-XWORD_LEN);
                xmatrix.mark_as_used(&extracted_word, curr_col, curr_col+XWORD_LEN, curr_row, curr_row-XWORD_LEN, &mut total);
            }
        }
    }

    dbg!(xmatrix.format());
    println!("Total XMAS occurrences: {total}");

}
