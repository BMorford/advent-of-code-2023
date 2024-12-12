use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

fn main() {
    let path = Path::new("../../input.txt");
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

        let current_value: Vec<char> = vec![
            chars[0],
            match chars.last().copied() {
                Some(c) => c,
                None => panic!(),
            },
        ];
        let current_value_number = match current_value
            .into_iter()
            .map(|c| c.to_digit(RADIX))
            .try_fold(0, |ans, i| i.map(|i| ans * RADIX + i))
        {
            None => panic!(),
            Some(n) => n,
        };
        result.push(current_value_number);
    }
    println!("Sum: {}", result.iter().sum::<u32>());
}
