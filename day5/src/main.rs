use std::fs::File;
use std::io::{BufRead, BufReader};
use colored::*;

fn main() {
    // read input_test.txt and call parse input
    let (paths, max_x, max_y) = parse_input("input.txt");
    println!("max_x: {}, max_y: {}", max_x, max_y);
    let mut matrix = vec![vec![0u16; (max_x + 1) as usize]; (max_y + 1) as usize];
    
    for path in paths {

        path.trace_path(&mut matrix);
        
        // println!("*********************");
        // println!("path: {:?} -> {:?}", path.start, path.end);
        // print_matrix(&matrix);
        // println!("*********************\n\n\n");
    
    }

    //print_matrix(&matrix);

    let mut count = 0;
    for row in matrix {
        for col in row {
            if col > 1 {
                count += 1;
            }
        }
    }

    println!("count: {}", count);
}

fn parse_input(filename: &str) -> (Vec<Path>, i32, i32) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut paths = Vec::new();
    let mut max_x = i32::min_value();
    let mut max_y = i32::min_value();
    
    for line in reader.lines() {
        let s = line.unwrap();
        if let Some(path) = Path::parse_paths(&s) {
            paths.push(path.clone());
            max_x = max_x.max(path.start.x).max(path.end.x);
            max_y = max_y.max(path.start.y).max(path.end.y);
        }
    }
    (paths, max_x, max_y)
}

fn print_matrix(matrix: &Vec<Vec<u16>>) {
    for row in matrix {
        for cell in row {
            if *cell == 0 {
                print!("{:1} ", cell);
            } else {
                print!("{:1} ", cell.to_string().red());
            }
        }
        println!();
    }
}

#[derive(Clone)]
// add derive(Debug) to print the struct
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Clone)]
struct Path {
    start: Point,
    end: Point,
}

impl Path {
    fn parse_paths(s: &str) -> Option<Path> {
        let parts: Vec<&str> = s.split(" -> ").collect();
        if parts.len() != 2 {
            return None;
        }
        let start_parts: Vec<&str> = parts[0].split(",").collect();
        let end_parts: Vec<&str> = parts[1].split(",").collect();
        if start_parts.len() != 2 || end_parts.len() != 2 {
            return None;
        }
        let start_x = start_parts[0].parse::<i32>().ok()?;
        let start_y = start_parts[1].parse::<i32>().ok()?;
        let end_x = end_parts[0].parse::<i32>().ok()?;
        let end_y = end_parts[1].parse::<i32>().ok()?;
        let start = Point { x: start_x, y: start_y };
        let end = Point { x: end_x, y: end_y };
        Some(Path { start, end })
    }

    fn trace_path(&self, matrix: &mut Vec<Vec<u16>>) {
        if self.start.y == self.end.y {
            // Horizontal path
            let y = self.start.y as usize;
            let start_x = self.start.x.min(self.end.x) as usize;
            let end_x = self.start.x.max(self.end.x) as usize;
            for x in start_x..=end_x {
                matrix[y][x] += 1;
            }
        } else if self.start.x == self.end.x {
            // Vertical path
            let x = self.start.x as usize;
            let start_y = self.start.y.min(self.end.y) as usize;
            let end_y = self.start.y.max(self.end.y) as usize;
            for y in start_y..=end_y {
                matrix[y][x] += 1;
            }
        } else {
            // Diagonal path - 45 degrees means dx = dy
            let steps = (self.end.x - self.start.x).abs() + 1;

            let start_x = self.start.x;
            let start_y = self.start.y;

            let is_up = if self.end.y > self.start.y { 1 } else { -1 }; 
            let is_right = if self.end.x > self.start.x { 1 } else { -1 };

            for i in 0..steps {
                let x = (start_x + is_right * i) as usize;
                let y = (start_y + is_up * i) as usize;
                matrix[y][x] += 1;
            }

        }
    }
}