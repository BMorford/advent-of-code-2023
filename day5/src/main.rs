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

    let seed_ids = lines[0]
        .split(' ')
        .skip(1)
        .map(|f| f.parse::<u32>().unwrap());
    let maps: Vec<String> = lines[2..].to_vec();

    let seeds = seed_ids.map(|id| Seed::new(id, &maps));

    let minimum_location_id = seeds.map(|x| x.location_id).min().unwrap();

    println!("Minimum location id: {}", minimum_location_id);
}

struct Seed {
    id: u32,
    soil_id: u32,
    fertilizer_id: u32,
    water_id: u32,
    light_id: u32,
    temperature_id: u32,
    humidity_id: u32,
    location_id: u32,
}

impl Seed {
    pub fn new(id: u32, maps: &Vec<String>) -> Self {
        {
            let soil_id = Self::get_map_value(id, "seed-to-soil", &maps);
            let fertilizer_id = Self::get_map_value(soil_id, "soil-to-fertilizer", &maps);
            let water_id = Self::get_map_value(fertilizer_id, "fertilizer-to-water", &maps);
            let light_id = Self::get_map_value(water_id, "water-to-light", &maps);
            let temperature_id = Self::get_map_value(light_id, "light-to-temperature", &maps);
            let humidity_id = Self::get_map_value(temperature_id, "temperature-to-humidity", &maps);
            let location_id = Self::get_map_value(humidity_id, "humidity-to-location", &maps);

            Seed {
                id,
                soil_id,
                fertilizer_id,
                water_id,
                light_id,
                temperature_id,
                humidity_id,
                location_id,
            }
        }
    }

    fn get_map_value(id: u32, key: &str, maps: &Vec<String>) -> u32 {
        let map_values = match maps.iter().position(|m| m.starts_with(key)) {
            Some(x) => maps[x + 1..].iter().take_while(|&m| !m.is_empty()),
            None => panic!("Invalid key"),
        };

        for val in map_values {
            if let [destination_range, source_range, range_length] = val
                .split(' ')
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()[..]
            {
                let range = source_range..=source_range + range_length;
                if range.contains(&id) {
                    return destination_range + (id - source_range);
                }
            }
        }
        id
    }
}
