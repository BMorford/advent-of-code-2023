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
    let lines = reader.lines();

    let mut games: Vec<Game> = vec![];
    for line in lines {
        games.push(Game::new(line.expect("")));
    }
    let valid_games = games.iter().filter(|f| f.is_valid);

    let part_one_result = valid_games.map(|f| f.id).sum::<u32>();
    println!("Sum: {}", part_one_result);

    // Part 2.
    let sum = games
        .iter()
        .map(|f| {
            let red_max = match f.red.iter().max() {
                None => &0,
                Some(x) => x,
            };
            let green_max = match f.green.iter().max() {
                None => &0,
                Some(x) => x,
            };
            let blue_max = match f.blue.iter().max() {
                None => &0,
                Some(x) => x,
            };
            red_max * green_max * blue_max
        })
        .sum::<u32>();

    println!("Part 2 sum: {}", sum);
}

struct Game {
    id: u32,
    red: Vec<u32>,
    green: Vec<u32>,
    blue: Vec<u32>,
    is_valid: bool,
}

impl Game {
    const RED_LIMIT: &'static u32 = &12;
    const GREEN_LIMIT: &'static u32 = &13;
    const BLUE_LIMIT: &'static u32 = &14;

    fn default() -> Game {
        Game {
            id: 0,
            red: vec![],
            green: vec![],
            blue: vec![],
            is_valid: true,
        }
    }

    pub fn new(game_values: String) -> Self {
        let mut new_game: Game = Game::default();
        new_game.get_game_id(&game_values);
        new_game.red = new_game.get_game_value(&game_values, " red", *Game::RED_LIMIT);
        new_game.green = new_game.get_game_value(&game_values, " green", *Game::GREEN_LIMIT);
        new_game.blue = new_game.get_game_value(&game_values, " blue", *Game::BLUE_LIMIT);
        new_game
    }

    fn get_game_id(&mut self, game_values: &String) {
        const RADIX: u32 = 10;
        let parts = game_values.split("Game ").collect::<Vec<&str>>();
        self.id = parts[1]
            .chars()
            .take_while(|f| f.is_digit(10))
            .map(|c| c.to_digit(RADIX))
            .try_fold(0, |ans, i| i.map(|i| ans * RADIX + i))
            .expect("")
    }

    fn get_game_value(&mut self, game_values: &String, value_marker: &str, limit: u32) -> Vec<u32> {
        let matches = game_values.match_indices(&value_marker);
        matches
            .map(|x| -> u32 {
                match game_values.split_at(x.0).0.rsplit_once(" ") {
                    Some(f) => {
                        let val = f.1.parse::<u32>().expect("");
                        if val > limit {
                            self.is_valid = false;
                        }
                        val
                    }
                    None => 0,
                }
            })
            .collect::<Vec<_>>()
    }
}
