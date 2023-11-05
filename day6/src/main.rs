use std::{fs::File, io::{BufReader, BufRead}, collections::HashMap};

fn main() {
    
    let lanternfish_vec = from_csv("input.csv");

    // convert to a map of key: counter, value: number of lanternfish
    let mut lanternfish_map: HashMap<u8, u64> = HashMap::new();
    for lanternfish in &lanternfish_vec {
        let counter = lanternfish.counter;
        let count = lanternfish_map.entry(counter).or_insert(0);
        *count += 1;
    }

    let num_days = 256;

    for _ in 0..num_days {
        
        let mut new_map = HashMap::new();

        for (counter, num_lanternfish) in &lanternfish_map {
            let mut fish = Lanternfish { counter: *counter };

            if let Some(new_lanternfish) = fish.step_one_day() {
                let count = new_map.entry(new_lanternfish.counter).or_insert(0);
                *count += num_lanternfish;
            }

            let count = new_map.entry(fish.counter.clone()).or_insert(0);
            *count += num_lanternfish;
        }

        lanternfish_map = new_map;
    
    }

    let total_fish = lanternfish_map.values().sum::<u64>();  
    println!("Number of lanternfish: {}", total_fish);
}

fn from_csv(filename: &str) -> Vec<Lanternfish> {
        
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut lanternfish_vec = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut counter_vec = Vec::new();

        for counter_str in line.split(',') {
            let counter = counter_str.parse().unwrap();
            counter_vec.push(counter);
        }

        for counter in counter_vec {
            let lanternfish = Lanternfish { counter };
            lanternfish_vec.push(lanternfish);
        }
    }

    lanternfish_vec
}

// Clonable
#[derive(Clone, Debug)]
struct Lanternfish {
    counter: u8,
}

impl Lanternfish {
    fn step_one_day(&mut self) -> Option<Lanternfish>{
        if self.counter == 0 {
            self.counter = 6;
            let new_lanternfish = Lanternfish { counter: 8 };
            return Some(new_lanternfish);
        } else {
            self.counter -= 1;
        }

        None
    }
}