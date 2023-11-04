
use std::{fs, collections::VecDeque};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let vectors = get_vectors(input);
    let (gamma_vec, sigma_vec) = calc_gamma_sigma(&vectors);

    let gamma_val = boolean_array_to_integer(&gamma_vec);
    let sigma_val = boolean_array_to_integer(&sigma_vec);

    println!("Ganma: {}, Sigma: {} Consumption: {}", gamma_val, sigma_val, sigma_val * gamma_val);

    let oxygen_rating = filter_list_of_vectors(&vectors, true);
    let co2_rating = filter_list_of_vectors(&vectors, false);

    println!("Oxygen * CO2: {}", 
        boolean_array_to_integer(&oxygen_rating) * 
        boolean_array_to_integer(&co2_rating));
}

fn get_vectors(input:String) -> Vec<Vec<bool>> {
    let num_chars_per_line = input.lines().next().unwrap().chars().count();
    
    let mut list_of_vectors: Vec<Vec<bool>> = std::iter::repeat_with(|| Vec::new())
        .take(num_chars_per_line)
        .collect();

    for line in input.lines() {
        for (i, c) in line.chars().enumerate() {
            let vector = &mut list_of_vectors[i];
            if c == '1' {
                vector.push(true);
            } else if c == '0' {
                vector.push(false);
            }
        }
    }

    list_of_vectors
}

fn filter_list_of_vectors(list_of_vectors: &Vec<Vec<bool>>, most_freq: bool) -> Vec<bool> {

    let num_col = list_of_vectors.len();
    let mut result = list_of_vectors.clone();

    for i in (0..num_col) {

        let mut gamma = calc_gamma_sigma(&result);
        let mask = if most_freq { &gamma.0 } else { &gamma.1 };
        let mut new_vals: Vec<Vec<bool>> = std::iter::repeat_with(|| Vec::new())
            .take(num_col)
            .collect();

        for j in 0..result[0].len() {
            if result[i][j] == mask[i] {
                
                for k in 0..num_col {
                    new_vals[k].push(result[k][j]); 
                }
            }
        }

        result = new_vals;
 
        if result[0].len() == 1 {
            break;
        }
    }

    result.iter().map(|x| x[0]).collect()
}

fn calc_gamma_sigma(list_of_vectors: &Vec<Vec<bool>>) -> (Vec<bool>, Vec<bool>) {

    let mut gamma_vec = Vec::new();
    let mut sigma_vec = Vec::new();

    for vector in list_of_vectors {
        let most_frequent_value = calculate_most_frequent_value(&vector);
        gamma_vec.push(most_frequent_value);
        sigma_vec.push(!most_frequent_value);
    }

    (gamma_vec, sigma_vec)
}

fn boolean_array_to_integer (vector: &Vec<bool>) -> u32 {
    let mut result = 0;
    for (i, value) in vector.iter().rev().enumerate() {
        if *value {
            result += 2_u32.pow(i as u32);
        }
    }
    return result;
}

fn calculate_most_frequent_value (vector: &Vec<bool>) -> bool {
    let mut num_true = 0;
    let mut num_false = 0;
    for value in vector {
        if *value {
            num_true += 1;
        } else {
            num_false += 1;
        }
    }
    if num_true >= num_false {
        return true;
    } else {
        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_one() {

        let input = "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010".to_string();
        let vectors = get_vectors(input);
        let (gamma_val, sigma_val) = calc_gamma_sigma(&vectors);
        assert_eq!(boolean_array_to_integer(&gamma_val), 22);
        assert_eq!(boolean_array_to_integer(&sigma_val), 9);

        let oxygen_rating = filter_list_of_vectors(&vectors, true);
        let co2_rating = filter_list_of_vectors(&vectors, false);

        assert_eq!(boolean_array_to_integer(&oxygen_rating), 23);
        assert_eq!(boolean_array_to_integer(&co2_rating), 10);
    }

    // Test boolean to integer
    #[test]
    fn test_boolean_array_to_integer() {
        let input = vec![true, false, true, true, false];
        let result = boolean_array_to_integer(&input);
        assert_eq!(result, 22);
    }
}
