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

    let mut part_numbers: Vec<_> = vec![];
    let mut potential_gears: Vec<_> = vec![];
    for (i, line) in part_grid.iter().enumerate() {
        for part in line {
            let test_values: Vec<(usize, usize, char)> = adjacent_values(&lines, i, part);

            if test_values
                .iter()
                .any(|v| (!v.2.is_numeric()) && v.2 != '.')
            {
                part_numbers.push(part.1);
                let gears = test_values.into_iter().filter(|v| v.2 == '*');
                gears.for_each(|g| potential_gears.push((g.0, g.1, part.1)))
            }
        }
    }

    let sum = part_numbers
        .iter()
        .map(|f| f.parse::<u32>().expect(""))
        .sum::<u32>();
    println!("Part 1 sum: {}", sum);

    // Part 2.
    let mut gear_ratio: u32 = 0;
    potential_gears.sort_by_key(|f| (f.0, f.1));

    for [a, b] in potential_gears.windows(2).flat_map(&<[_; 2]>::try_from) {
        if a.0 == b.0 && a.1 == b.1 {
            gear_ratio += a.2.parse::<u32>().expect("") * b.2.parse::<u32>().expect("");
        }
    }
    println!("Ratio sum: {}", gear_ratio);
}

fn slice_index(s: &String, m: &(usize, &str)) -> (String, usize) {
    let char_count = m.1.len();

    let upper_index = if (m.0 + char_count) < s.len() {
        m.0 + char_count
    } else {
        m.0 + char_count - 1
    };
    let lower_index = m.0.checked_sub(1).map_or(m.0, |f| f);

    (s[lower_index..=upper_index].to_string(), lower_index)
}

fn adjacent_values(
    grid: &Vec<String>,
    y_index: usize,
    part: &(usize, &str),
) -> Vec<(usize, usize, char)> {
    let line_chars = grid[y_index].chars().collect::<Vec<_>>();
    let mut values: Vec<(usize, usize, char)> = vec![];
    let char_count = part.1.len();

    if Option::is_some(&part.0.checked_sub(1)) {
        values.push((part.0 - 1, y_index, line_chars[part.0 - 1]));
    }

    if part.0 + char_count < line_chars.len() {
        values.push((
            part.0 + char_count,
            y_index,
            line_chars[part.0 + char_count],
        ));
    }

    if y_index != 0 {
        let slice = slice_index(&grid[y_index - 1], part);
        values.extend(
            slice
                .0
                .chars()
                .enumerate()
                .map(|f| (slice.1 + f.0, y_index - 1, f.1)),
        )
    }

    if y_index + 1 < grid.len() {
        let slice = slice_index(&grid[y_index + 1], part);
        values.extend(
            slice
                .0
                .chars()
                .enumerate()
                .map(|f| (slice.1 + f.0, y_index + 1, f.1)),
        )
    }

    return values;
}
