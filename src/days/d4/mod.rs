use crate::common;

const X_WORD: &str = "XMAS";
const XWORD_LEN: usize = X_WORD.len() - 1;

// Transpose matrix
fn transpose<T>(original: &Vec<Vec<T>>) -> Vec<Vec<&T>> {
    assert!(!original.is_empty());
    let mut transposed = (0..original[0].len()).map(|_| vec![]).collect::<Vec<_>>();

    for original_row in original {
        for (item, transposed_row) in original_row.into_iter().zip(&mut transposed) {
            transposed_row.push(item);
        }
    }

    transposed
}

pub fn result () {
    let mut total = 0;
    let input_data = common::read_content_from_file("d4_p1__input.txt");
    let mut xma_trix: Vec<Vec<char>> = input_data.lines().map(|line| line.chars().collect()).collect();


    for row in xma_trix.iter().enumerate() {
        for col in row.1.iter().enumerate() {
            // Left
            if let Some(left) = col.0.checked_sub(XWORD_LEN) {
                total += if X_WORD.eq(row.1[left..=(col.0)].iter().rev().collect::<String>().as_str()) { 1 } else { 0 };
            }
            // Right
            if col.0 + XWORD_LEN <= row.1.len()-1 {
                let right = col.0 + XWORD_LEN;
                let word = row.1[(col.0)..=right].iter().collect::<String>();
                total += if X_WORD.eq(word.as_str()) { 1 } else { 0 };
            }
            // Up
            if let Some(up) = row.0.checked_sub(XWORD_LEN) {
                let word = xma_trix[up..=(up+XWORD_LEN)].iter().rev().map(|x| x[col.0]).collect::<String>();
                total += if X_WORD.eq(word.as_str()) { 1 } else { 0 };
            }
            // Down
            if row.0 + XWORD_LEN <= xma_trix.len()-1 {
                let down = row.0 + XWORD_LEN;
                let word = xma_trix[row.0..=(down)].iter().map(|x| x[col.0]).collect::<String>();
                total += if X_WORD.eq(word.as_str()) { 1 } else { 0 };
            }
            // DiagonalUpLeft
            if col.0.checked_sub(XWORD_LEN).is_some() && row.0.checked_sub(XWORD_LEN).is_some() {
                let mut word = String::new();
                let mut dec_col = col.0;
                xma_trix[(row.0-XWORD_LEN)..=row.0].iter().rev().for_each(|letters| {
                    word.push(letters[dec_col]);
                    dec_col -= if dec_col > 0 { 1 } else { 0 };
                });
                total += if X_WORD.eq(word.as_str()) { 1 } else { 0 };
            }

            // DiagonalUpRight
            if col.0 + XWORD_LEN <= row.1.len()-1 && row.0.checked_sub(XWORD_LEN).is_some() {
                let mut word = String::new();
                let mut asc_col = col.0;
                xma_trix[(row.0-XWORD_LEN)..=row.0].iter().rev().for_each(|letters| {
                   word.push(letters[asc_col]);
                   asc_col += if asc_col < row.1.len()-1 { 1 } else { 0 };
                });
                total += if X_WORD.eq(word.as_str()) { 1 } else { 0 };
            }

            // DiagonalDownLeft
            if col.0.checked_sub(XWORD_LEN).is_some() && row.0 + XWORD_LEN <= row.1.len()-1 {
                let mut word = String::new();
                let mut dec_col = col.0;
                xma_trix[row.0..=(row.0 + XWORD_LEN)].iter().for_each(|letters| {
                    word.push(letters[dec_col]);
                    dec_col -= if dec_col > 0 { 1 } else { 0 };
                });
                total += if X_WORD.eq(word.as_str()) { 1 } else { 0 };
            }

            // DiagonalDownRight
            if col.0 + XWORD_LEN <= row.1.len()-1 && row.0 + XWORD_LEN <= row.1.len()-1 {
                let mut word = String::new();
                let mut asc_col = col.0;
                xma_trix[row.0..=(row.0 + XWORD_LEN)].iter().for_each(|letters| {
                    if asc_col < row.1.len() {
                        word.push(letters[asc_col]);
                        asc_col += 1;
                    }
                });
                total += if X_WORD.eq(word.as_str()) { 1 } else { 0 };
            }
        }
    }
    println!("D4 (part 1)  :: Total of XMAS: {total}");
    println!("D4 (part 2)  :: Total of X-MAS: {}", part2(&xma_trix));
}

fn part2(xma_trix: &Vec<Vec<char>>) -> usize {
    const MAS_WORDS: [&str; 2] = ["MAS", "SAM"];
    const MASWORD_LEN: usize = MAS_WORDS[0].len() - 1;

    let mut total_x = 0;

    for row in xma_trix.iter().enumerate() {
        for col in row.1.iter().enumerate() {
            // DiagonalDownRight
            if col.0 + MASWORD_LEN <= row.1.len()-1 && row.0 + MASWORD_LEN <= row.1.len()-1 {
                let mut word = String::new();
                let mut asc_col = col.0;
                xma_trix[row.0..=(row.0 + MASWORD_LEN)].iter().for_each(|letters| {
                    if asc_col < row.1.len() {
                        word.push(letters[asc_col]);
                        asc_col += 1;
                    }
                });
                // If found the first diagonal "MAS" (or  "SAM") check for the second one.
                if MAS_WORDS.contains(&word.as_str()) {
                    let mut word = String::new();
                    let mut dec_col = col.0+MASWORD_LEN;
                    xma_trix[row.0..=(row.0 + MASWORD_LEN)].iter().for_each(|letters| {
                        word.push(letters[dec_col]);
                        dec_col -= if dec_col > 0 { 1 } else { 0 };
                    });
                    if MAS_WORDS.contains(&word.as_str()) {
                        total_x += 1;
                    }
                }
            }
        }
    }

    total_x
}

