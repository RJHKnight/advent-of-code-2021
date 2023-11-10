use std::collections::{HashSet, VecDeque, HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader};
use rand::Rng;

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let heightmap: Vec<Vec<u32>> = reader
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|num| num.to_digit(10).unwrap())
                .collect()
        })
        .collect();

    let mut lowest_values: HashSet<(usize, usize)> = HashSet::new();

    for y in 0..heightmap.len() {
        for x in 0..heightmap[y].len() {
            let mut is_lowest = true;
            let value = heightmap[y][x];

            for direction in &[Direction::Up, Direction::Down, Direction::Left, Direction::Right] {
                if let Some(neighbor_value) = get_neighbour_value(&heightmap, x, y, direction) {
                    if neighbor_value <= value {
                        is_lowest = false;
                        break;
                    }
                }
            }

            if is_lowest {
                lowest_values.insert((x, y));
            }
        }
    }

    let mut low_sum: u32 = 0;

    for low_point in lowest_values.clone() {
        low_sum += heightmap[low_point.1][low_point.0] +1
    }

    println!("Lowest values sum: {}", low_sum);

    let mut connected_values: HashMap<(usize, usize), usize> = HashMap::new();
    let mut remaining_low_points: HashSet<(usize, usize)> = lowest_values.clone();

    while !remaining_low_points.is_empty() {
        
        let start_point = *remaining_low_points.iter().next().unwrap();
        remaining_low_points.remove(&start_point);

        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
        queue.push_back(start_point);

        let mut connected: HashSet<(usize, usize)> = HashSet::new();

        while let Some(point) = queue.pop_front() {
            visited.insert(point);

            connected.insert(point);

            for direction in &[Direction::Up, Direction::Down, Direction::Left, Direction::Right] {
                if let Some(neighbor_value) = get_neighbour_value(&heightmap, point.0, point.1, direction) {

                    if neighbor_value == 9 {
                        continue;
                    }

                    let neighbor_point = get_neighbour_point(point.0, point.1, direction);

                    if !visited.contains(&neighbor_point) {
                        
                        if remaining_low_points.contains(&neighbor_point) {
                            remaining_low_points.remove(&neighbor_point);
                        }
                        queue.push_back(neighbor_point);
                    }
                }
            }
        }

        connected_values.insert(start_point, connected.len());
    }

    for (key, value) in &connected_values {
        println!("***** {:?} *****", key);
        println!("{:?}", value);
    }

    // sort connected values on the value and multiply the 3 largest values
    let mut sizes: Vec<usize> = connected_values.values().cloned().collect();

    sizes.sort();
    sizes.reverse();

    let result :usize = sizes.iter().take(3).product();
    
    println!("Product: {}", result);
    


}

const SAMPLE_SIZE: usize = 40;

fn print_sample(heightmap: &Vec<Vec<u32>>, lowest_vals: &HashSet<(usize, usize)>) {
        // Get a random starting point for the sample
    let mut rng = rand::thread_rng();
    let start_x = rng.gen_range(0..heightmap[0].len() - SAMPLE_SIZE);
    let start_y = rng.gen_range(0..heightmap.len() - SAMPLE_SIZE);

    // Print the sample
    for y in start_y..start_y + SAMPLE_SIZE {
        for x in start_x..start_x + SAMPLE_SIZE {
            let value = heightmap[y][x];
            let is_lowest = lowest_vals.contains(&(x, y));

            if is_lowest {
                print!("\x1b[31m{}\x1b[0m ", value);
            } else {
                print!("{} ", value);
            }
        }
        println!();
    }
}

enum Direction{
    Up,
    Down,
    Left,
    Right,
}

fn get_neighbour_point(x: usize, y: usize, direction: &Direction) -> (usize, usize) {
    match direction {
        Direction::Up => (x, y - 1),
        Direction::Down => (x, y + 1),
        Direction::Left => (x - 1, y),
        Direction::Right => (x + 1, y),
    }
}

fn get_neighbour_value(heightmap: &Vec<Vec<u32>>, x: usize, y: usize, direction: &Direction) -> Option<u32> {
    match direction {
        Direction::Up => {
            if y == 0 {
                None
            } else {
                Some(heightmap[y - 1][x])
            }
        }
        Direction::Down => {
            if y == heightmap.len() - 1 {
                None
            } else {
                Some(heightmap[y + 1][x])
            }
        }
        Direction::Left => {
            if x == 0 {
                None
            } else {
                Some(heightmap[y][x - 1])
            }
        }
        Direction::Right => {
            if x == heightmap[y].len() - 1 {
                None
            } else {
                Some(heightmap[y][x + 1])
            }
        }
    }
}