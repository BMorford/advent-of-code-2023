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

    let mut sum: u32 = 0;
    for line in lines {
        let (winning_numbers, picked_numbers) =
            match &line.split(": ").collect::<Vec<&str>>()[1].split_once('|') {
                Some(x) => (
                    x.0.split_whitespace(),
                    x.1.split_whitespace().collect::<Vec<_>>(),
                ),
                _ => panic!(),
            };

        let match_count = winning_numbers
            .filter(|f| picked_numbers.contains(f))
            .count();

        sum += 2_u32.pow(match_count as u32 - 1);
    }
    println!("Score: {}", sum);
}
