use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // Open the file and read the single line
    let file = File::open("input.csv").unwrap();
    let line = BufReader::new(file).lines().next().unwrap().unwrap();

    // Parse the line into a vector of i32
    let vec: Vec<i32> = line.split(',').map(|s| s.parse().unwrap()).collect();

    let min_val = min_total_fuel(vec);

    // Print the result
    println!("Best position: {} with sum {}", min_val.0, min_val.1);
}

fn min_total_fuel(vec: Vec<i32>) -> (i32, i32) {
    // Find the min and max values
    let min = *vec.iter().min().unwrap();
    let max = *vec.iter().max().unwrap();

    let mut fuel_cost = std::collections::HashMap::new();
    for i in min..=max {
        let sum :i32 = (1..=i).sum();
        fuel_cost.insert(i, sum);
    }

    // Loop through the possible values and calculate the sum of abs diff
    let mut min_sum = std::i32::MAX;
    let mut min_val = 0;

    for i in min..=max {
        let sum: i32 = vec.iter().map(|&x| fuel_cost.get(&(x - i).abs()).unwrap()).sum();
        if sum < min_sum {
            min_sum = sum;
            min_val = i;
        }
    }
    (min_val, min_sum)
}

// Test case for min fuel using values 16,1,2,0,4,2,7,1,2,14
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_total_fuel() {
        let vec = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        let res = min_total_fuel(vec);
        assert_eq!(res.1, 168);
        assert_eq!(res.0, 5);
    }
}