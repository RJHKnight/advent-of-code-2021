use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::usize;
use colored::*;

fn main() {
    let mut grid = Grid::from_file("input.txt").unwrap();
    grid = expand_grid(grid, 4);

    let path = grid.a_star(Cursor { x: 0, y: 0 }, Cursor { x: grid.data.len()-1, y: grid.data[0].len()-1 });

    //print_grid_with_path(&grid, &path);

    let mut sum = 0;

    for i in 0..path.len()-1 {
        sum += grid.cost(path[i+1]);
    }

    println!("Total cost: {}", sum);
}

fn expand_grid(grid: Grid, times: usize) -> Grid{
    
    // First expand the grid below
    let mut working_grid = grid.clone();
    let mut new_grid = grid.clone();

    for _ in 0..times {
        increase_grid(&mut working_grid);
        new_grid = increase_grid_below(&new_grid, &working_grid);
    }

    working_grid = new_grid.clone();

    for _ in 0..times {
        increase_grid(&mut working_grid);
        new_grid = increase_grid_right(&new_grid, &working_grid);
    }

    new_grid
}

fn increase_grid(grid: &mut Grid) {
    for i in 0..grid.data.len() {
        for j in 0..grid.data[i].len() {
            let mut new_val = grid.data[i][j] + 1;
            if new_val > 9 {
                new_val = 1;
            }
            grid.data[i][j] = new_val;
        }
    }
}

fn increase_grid_below(grid: &Grid, new_grid: &Grid) -> Grid {
    let mut result = grid.data.clone();
    result.append(&mut new_grid.data.clone());

    Grid{data: result}
}

fn increase_grid_right(grid: &Grid, new_grid: &Grid) -> Grid {
    let mut result = grid.data.clone();
    for i in 0..result.len() {
        result[i].append(&mut new_grid.data[i].clone());
    }

    Grid{data: result}
}


fn print_grid_with_path(grid: &Grid, path: &Vec<Cursor>) {
    for (y, row) in grid.data.iter().enumerate() {
        for (x, &value) in row.iter().enumerate() {
            let cursor = Cursor { x, y };
            if path.contains(&cursor) {
                print!("{}", value.to_string().red());
            } else {
                print!("{}", value);
            }
        }
        println!();
    }
}
#[derive(Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: Cursor,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Grid {
    fn a_star(&self, start: Cursor, goal: Cursor) -> Vec<Cursor> {
        let mut open_set = BinaryHeap::new();
        open_set.push(State { cost: 0, position: start });

        let mut came_from = HashMap::new();
        let mut g_score = HashMap::new();
        g_score.insert(start, 0);

        let mut f_score = HashMap::new();
        f_score.insert(start, self.heuristic_cost_estimate(start, goal));

        while let Some(State { cost, position }) = open_set.pop() {
            if position == goal {
                return self.reconstruct_path(came_from, position);
            }

            for &neighbor in self.neighbors(position).iter() {
                let tentative_g_score = g_score.get(&position).unwrap() + self.cost(neighbor);
                if tentative_g_score < *g_score.get(&neighbor).unwrap_or(&usize::MAX) {
                    came_from.insert(neighbor, position);
                    g_score.insert(neighbor, tentative_g_score);
                    f_score.insert(neighbor, tentative_g_score + self.heuristic_cost_estimate(neighbor, goal));
                    open_set.push(State { cost: f_score[&neighbor], position: neighbor });
                }
            }
        }

        Vec::new()
    }

    fn heuristic_cost_estimate(&self, start: Cursor, goal: Cursor) -> usize {
        let squared_val = (goal.x - start.x).pow(2) + (goal.y - start.y).pow(2);
        (squared_val as f64).sqrt() as usize
    }

    fn cost(&self, to: Cursor) -> usize {
        self.data[to.y][to.x] as usize
    }

    fn neighbors(&self, position: Cursor) -> Vec<Cursor> {
        // Implement the function to get the neighbors of a position here.
        let mut neighbors = Vec::new();
        for &(x, y) in [(0, -1), (0, 1), (-1, 0), (1, 0)].iter() {
            let neighbor = Cursor {
                x: (position.x as isize + x) as usize,
                y: (position.y as isize + y) as usize,
            };
            if neighbor.x < self.data.len() && neighbor.y < self.data.len() {
                neighbors.push(neighbor);
            }
        }
        neighbors
    }

    fn reconstruct_path(&self, came_from: HashMap<Cursor, Cursor>, goal: Cursor) -> Vec<Cursor> {
        // Implement the function to reconstruct the path from start to goal here.

        let mut path = Vec::new();
        let mut current = goal;
        while let Some(&prev) = came_from.get(&current) {
            path.push(current);
            current = prev;
        }
        path.push(current);
        path.reverse();
        path
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Cursor {
    x: usize,
    y: usize,
}

#[derive(Clone)]
struct Grid {
    data: Vec<Vec<i8>>,
}

impl Grid {

    fn from_file<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let file = File::open(&path)?;
        let reader = io::BufReader::new(file);

        let mut data = Vec::new();

        for line in reader.lines() {
            let line = line?;
            let row: Vec<i8> = line.chars().map(|c| c.to_digit(10).unwrap() as i8).collect();
            data.push(row);
        }

        Ok(Self {
            data,
        })
    }
}