use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

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
    let lines = reader.lines();

    let mut result: Vec<u32> = vec![];
    for line in lines {
        const RADIX: u32 = 10;
        let chars: Vec<char> = line
            .unwrap()
            .chars()
            .filter(|c| c.is_digit(RADIX))
            .collect();

        let current_value: Vec<char> = vec![chars[0], chars.last().copied().expect("")];
        let current_value_number = current_value
            .into_iter()
            .map(|c| c.to_digit(RADIX))
            .try_fold(0, |ans, i| i.map(|i| ans * RADIX + i))
            .expect("");
        result.push(current_value_number);
    }
    println!("Sum: {}", result.iter().sum::<u32>());
    part2();
}

fn part2() {
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
    let lines = reader.lines();
    let mut result: Vec<u32> = vec![];
    for line in lines {
        const RADIX: u32 = 10;

        let current_line = line.unwrap();
        let number_words = [
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3",
            "4", "5", "6", "7", "8", "9",
        ];

        let mut matches: Vec<(usize, &str)> = vec![];

        for number_word in number_words.iter() {
            let curr_match: Vec<(usize, &str)> = current_line.match_indices(number_word).collect();
            matches.extend(curr_match);
        }

        let current_value: Vec<String> = vec![
            replace_number_word(String::from(matches.iter().min_by_key(|x| x.0).unwrap().1)),
            replace_number_word(String::from(matches.iter().max_by_key(|x| x.0).unwrap().1)),
        ];

        let current_value_number = current_value
            .into_iter()
            .map(|x| x.chars().next().expect("").to_digit(RADIX))
            .try_fold(0, |ans, i| i.map(|i| ans * RADIX + i))
            .expect("");
        result.push(current_value_number);
    }
    println!("Sum: {}", result.iter().sum::<u32>());
}

fn replace_number_word(value: String) -> String {
    const NUMBER_WORDS: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut return_value = value;

    for (i, number_word) in NUMBER_WORDS.iter().enumerate() {
        return_value = return_value.replace(number_word, &(i + 1).to_string());
    }
    return return_value;
}
