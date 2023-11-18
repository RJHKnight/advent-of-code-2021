use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use strum::IntoEnumIterator; // 0.17.1
use strum_macros::EnumIter; // 0.17.1
use colored::*;

fn main() {
    let mut res = read_file_to_vec("input.txt").unwrap();
    let mut total_flashed = 0;

    let mut i = 0;
    while true {
        i += 1;
        let (new_grid, flashed) = step(res);

        total_flashed += flashed;
        res = new_grid;

        // if all values are equal to 0, print the line number
        let mut all_zero = true;
        for row in res.iter() {
            for &value in row.iter() {
                if value != 0 {
                    all_zero = false;
                }
            }
        }

        if all_zero {
            println!("All zero at step {}", i);
            break;
        }

        if i % 10 == 0 {
            //print_res(&res);
            //println!("Step {} flashed {}", i, flashed);
        }
    
    }

    println!("Total flashed {}", total_flashed);

}

fn print_res(res: &Vec<Vec<i32>>) {
    for row in res {
        for &value in row {
            if value == 0 {
                print!("{}", value.to_string().red());
            } else {
                print!("{}", value);
            }
            print!(" ");
        }
        println!();
    }
}

fn step(grid: Vec<Vec<i32>>) -> (Vec<Vec<i32>>, i32) {
    
    let mut new_grid = grid.clone();
    let mut flashed = 0;

    // First increment all the values by 1
    for row in new_grid.iter_mut() {
        for val in row.iter_mut() {
            *val += 1;
        }
    }

    let mut flashed_points = Vec::new();

    // Check for any values greater than 9
    for y in 0..new_grid.len() {
        let row = new_grid.get(y).unwrap();
        for x in 0..row.len() {

            let val = row.get(x).unwrap();    
            if *val > 9 {
                flashed += 1;
                flashed_points.push((x, y));
            }
        }
    }

    while !flashed_points.is_empty() {
        // Loop over all the neighbors of the flashed point, incrementing its value by 1. 
        // If the new value is equal to 10, increment flashed and add the neighbouring point to flashed_points

        // loop over all values of direction
        let next_point = flashed_points.pop();
        
        if next_point.is_some(){
            let (x, y) = next_point.unwrap();
                
                for dir in Direction::iter() {
                    let neighbour = get_neighbour(&new_grid, x, y, &dir);
                    if neighbour.is_some() {
                        // Cascade
                        if neighbour.unwrap() == 9 {
                            flashed += 1;
                            flashed_points.push(get_neightbour_coords(x, y, &dir));
                        }
                        // Increment neighbour
                        let (nx, ny) = get_neightbour_coords(x, y, &dir);
                        let neighbour = new_grid.get_mut(ny).unwrap().get_mut(nx).unwrap();
                        *neighbour += 1;

                    }
            }
        }
    }

    // Reset values > 9 to 0
    for y in 0..new_grid.len() {
        let row = new_grid.get_mut(y).unwrap();
        for x in 0..row.len() {
            let val = row.get_mut(x).unwrap();    
            if *val > 9 {
                *val = 0;
            }
        }
    }

    (new_grid, flashed)

}

fn read_file_to_vec<P>(filename: P) -> io::Result<Vec<Vec<i32>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    let mut data = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let numbers: Vec<i32> = line.chars().map(|c| c.to_digit(10).unwrap() as i32).collect();
        data.push(numbers);
    }

    Ok(data)
}

fn get_neightbour_coords(x: usize, y: usize, dir : &Direction) -> (usize, usize) {
    match dir {
        Direction::Up => (x, y-1),
        Direction::Down => (x, y+1),
        Direction::Left => (x-1, y),
        Direction::Right => (x+1, y),
        Direction::DiagUpLeft => (x-1, y-1),
        Direction::DiagUpRight => (x+1, y-1),
        Direction::DiagDownLeft => (x-1, y+1),
        Direction::DiagDownRight =>  (x+1, y+1),
    }
}

fn get_neighbour(data: &Vec<Vec<i32>>, x: usize, y: usize, dir : &Direction) -> Option<i32> {
    match dir {
        Direction::Up => get_value_with_delta(data, x, y, 0, -1),
        Direction::Down => get_value_with_delta(data, x, y, 0, 1),
        Direction::Left => get_value_with_delta(data, x, y, -1, 0),
        Direction::Right => get_value_with_delta(data, x, y, 1, 0),
        Direction::DiagUpLeft => get_value_with_delta(data, x, y, -1, -1),
        Direction::DiagUpRight => get_value_with_delta(data, x, y, 1, -1),
        Direction::DiagDownLeft => get_value_with_delta(data, x, y, -1, 1),
        Direction::DiagDownRight => get_value_with_delta(data, x, y, 1, 1),
    }
}

// enum for directions
#[derive(Debug, EnumIter)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    DiagUpLeft,
    DiagUpRight,
    DiagDownLeft,
    DiagDownRight,
}

fn get_value(data: &Vec<Vec<i32>>, x: usize, y: usize) -> Option<i32> {
    data.get(y).and_then(|row| row.get(x).cloned())
}

fn get_value_with_delta(data: &Vec<Vec<i32>>, x: usize, y: usize, dx: i32, dy: i32) -> Option<i32> {

    let new_x = x as i32 + dx;
    let new_y = y as i32 + dy;

    if new_x >= 0 && new_x < data.len() as i32 && new_y >= 0 && new_y < data.len() as i32 {
        get_value(data, new_x as usize, new_y as usize)
    } else {
        None
    }
}