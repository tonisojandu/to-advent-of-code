use num_bigint::BigInt;

pub fn solve(lines: Vec<String>, _: i32) {
    let unexpande: Vec<Vec<char>> = lines.iter().map(|line| {
        line.chars().collect::<Vec<char>>()
    }).collect();

    let mut empty_rows = vec![];
    let mut empty_cols = vec![];

    for y in 0..unexpande.len() {
        let mut empty_row = true;
        for x in 0..unexpande[y].len() {
            if unexpande[y][x] != '.' {
                empty_row = false;
                break;
            }
        }
        if empty_row {
            empty_rows.push(y);
        }
    }

    for x in 0..unexpande[0].len() {
        let mut empty_col = true;
        for y in 0..unexpande.len() {
            if unexpande[y][x] != '.' {
                empty_col = false;
                break;
            }
        }
        if empty_col {
            empty_cols.push(x);
        }
    }

    let mut galaxy_coords = vec![];
    for y in 0..unexpande.len() {
        for x in 0..unexpande[y].len() {
            if unexpande[y][x] == '#' {
                galaxy_coords.push((y as i64, x as i64));
            }
        }
    }

    let mut sum = 0;
    let mut big_sum = BigInt::from(0i64);
    for i in 0..galaxy_coords.len() {
        for j in (i + 1)..galaxy_coords.len() {
            let mut num_empty_spaces_between = 0;
            for k in 0..empty_rows.len() {
                let larger = galaxy_coords[i].0.max(galaxy_coords[j].0);
                let smaller = galaxy_coords[i].0.min(galaxy_coords[j].0);
                if empty_rows[k] > smaller as usize && empty_rows[k] < larger as usize {
                    num_empty_spaces_between += 1;
                }
            }
            for k in 0..empty_cols.len() {
                let larger = galaxy_coords[i].1.max(galaxy_coords[j].1);
                let smaller = galaxy_coords[i].1.min(galaxy_coords[j].1);
                if empty_cols[k] > smaller as usize && empty_cols[k] < larger as usize {
                    num_empty_spaces_between += 1;
                }
            }

            sum += (galaxy_coords[i].0 - galaxy_coords[j].0).abs() + (galaxy_coords[i].1 - galaxy_coords[j].1).abs() + num_empty_spaces_between as i64;
            big_sum = big_sum + BigInt::from((galaxy_coords[i].0 - galaxy_coords[j].0).abs() + (galaxy_coords[i].1 - galaxy_coords[j].1).abs())
                + (BigInt::from(999_999) * BigInt::from(num_empty_spaces_between as i64));
        }
    }

    println!("Day 11: 1 = {:?}", sum);
    println!("Day 11: 2 = {:?}", big_sum);
}