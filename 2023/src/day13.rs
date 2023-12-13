struct Note {
    pattern: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

impl Note {

    fn new(pattern: Vec<Vec<char>>) -> Note {
        let width = pattern[0].len();
        let height = pattern.len();
        Note {
            pattern,
            width,
            height,
        }
    }

    fn rows_differ_by(&self, row1: usize, row2: usize) -> i64 {
        let mut diffs = 0;
        for i in 0..self.pattern[row1].len() {
            if self.pattern[row1][i] != self.pattern[row2][i] {
                diffs += 1;
            }
        }
        return diffs;
    }

    fn cols_differ_by(&self, col1: usize, col2: usize) -> i64 {
        let mut diffs = 0;
        for i in 0..self.pattern.len() {
            if self.pattern[i][col1] != self.pattern[i][col2] {
                diffs += 1;
            }
        }
        return diffs;
    }
}

pub fn solve(lines: Vec<String>, part: i32) {
    let sum: i64 = lines.split(|line| line.len() == 0).map(|note_lines| {
        let note = Note::new(note_lines.iter().map(|line| {
            line.chars().collect::<Vec<char>>()
        }).collect::<Vec<Vec<char>>>());

        let mut most_row_reflections: i64 = 0;
        let mut row_reflected_on = usize::MAX;
        for y in 0..(note.height - 1) {
            let diff = note.rows_differ_by(y, y + 1);
            if (part == 2 && diff < 2) || (part == 1 && diff == 0) {
                let mut is_reflection = true;
                let mut reflected_on = 1;
                let mut off_by_ones = diff;
                for y2 in (0..y).rev() {
                    let other_side = y + (y - y2) + 1;
                    if other_side >= note.height {
                        break;
                    }
                    let other_diff = note.rows_differ_by(y2, other_side);
                    if part == 2 && (other_diff > 1 || off_by_ones + other_diff > 1) || (part == 1 && other_diff > 0) {
                        is_reflection = false;
                        break;
                    }
                    off_by_ones += other_diff;
                    reflected_on += 1;
                }
                if ((part == 2 && off_by_ones == 1) || part == 1) && is_reflection && reflected_on > most_row_reflections {
                    most_row_reflections = reflected_on;
                    row_reflected_on = y;
                }
            }
        }

        let mut most_col_reflections: i64 = 0;
        let mut col_reflected_on = usize::MAX;
        for x in 0..(note.width - 1) {
            let diff = note.cols_differ_by(x, x + 1);
            if (part == 2 && diff < 2) || (part == 1 && diff == 0) {
                let mut is_reflection = true;
                let mut reflected_on = 1;
                let mut off_by_ones = diff;
                for x2 in (0..x).rev() {
                    let other_side = x + (x - x2) + 1;
                    if other_side >= note.width {
                        break;
                    }
                    let other_diff = note.cols_differ_by(x2, other_side);
                    if part == 2 && (other_diff > 1 || off_by_ones + other_diff > 1) || (part == 1 && other_diff > 0) {
                        is_reflection = false;
                        break;
                    }
                    off_by_ones += other_diff;
                    reflected_on += 1;
                }
                if ((part == 2 && off_by_ones == 1) || part == 1) && is_reflection && reflected_on > most_col_reflections {
                    most_col_reflections = reflected_on;
                    col_reflected_on = x;
                }
            }
        }

        let col_result = if col_reflected_on != usize::MAX { col_reflected_on as i64 + 1 } else { 0i64 };
        let row_result = if row_reflected_on != usize::MAX { 100 * (row_reflected_on as i64 + 1) } else { 0i64 };
        col_result + row_result
    }).sum();

    println!("Day 13: {} = {:?}", part, sum);
}