use std::io::BufRead;
use std::{error::Error, fs::File, io::BufReader, path::Path};

fn main() {
    let path = Path::new("src/input.txt");
    let file = match File::open(&path) {
        Err(why) => panic!(
            "couldn't open {}: {}",
            path.display(),
            <dyn Error>::to_string(&why)
        ),
        Ok(file) => file,
    };

    let reader: BufReader<File> = BufReader::new(file);
    let lines = reader
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect::<Vec<_>>();

    let part_grid = lines
        .iter()
        .map(|line| {
            line.split(|c: char| !c.is_numeric())
                .filter(|c| !c.is_empty())
                .map(move |f| (f.as_ptr() as usize - line.as_ptr() as usize, f))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut part_numbers: Vec<u32> = vec![];
    for (i, line) in part_grid.iter().enumerate() {
        for part in line {
            let test_values: Vec<char> = adjacent_values(&lines, i, part);

            if test_values.iter().any(|v| (!v.is_numeric()) && v != &'.') {
                part_numbers.push(part.1.parse::<u32>().expect(""));
            }
        }
    }

    let sum = part_numbers.iter().sum::<u32>();
    println!("Part 1 sum: {}", sum);

    // Part 2.
}

fn slice_index(s: &String, m: &(usize, &str)) -> String {
    let char_count = m.1.len();

    let upper_index = if (m.0 + char_count) < s.len() {
        m.0 + char_count
    } else {
        m.0 + char_count - 1
    };
    let lower_index = m.0.checked_sub(1).map_or(m.0, |f| f);

    s[lower_index..=upper_index].to_string()
}

fn adjacent_values(grid: &Vec<String>, y_index: usize, part: &(usize, &str)) -> Vec<char> {
    let line_chars = grid[y_index].chars().collect::<Vec<_>>();
    let mut values: Vec<char> = vec![];
    let char_count = part.1.len();

    if Option::is_some(&part.0.checked_sub(1)) {
        values.push(line_chars[part.0 - 1]);
    }

    if part.0 + char_count < line_chars.len() {
        values.push(line_chars[part.0 + char_count]);
    }

    if y_index != 0 {
        values.extend(slice_index(&grid[y_index - 1], part).chars())
    }

    if y_index + 1 < grid.len() {
        values.extend(slice_index(&grid[y_index + 1], part).chars())
    }

    return values;
}
