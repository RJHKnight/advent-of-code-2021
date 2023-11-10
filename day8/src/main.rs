use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let lookup = create_map();
    let input = read_and_parse_file("input.txt");
    
    for panel in input {

        let this_mapping : HashMap<char, char> = parse(panel.0);

        let mut output_vals = Vec::new();

        for entry in panel.1 {
            let mut remapped_chars = Vec::new();

            for c in entry.chars() {
                if let Some(mapped_c) = this_mapping.get(&c) {
                    remapped_chars.push(*mapped_c);
                }
            }

            output_vals.push(lookup_sorted_vec(&lookup, remapped_chars).unwrap());
        }

        println!("{:?}", output_vals);
    }

}


fn read_and_parse_file(file_path: &str) -> Vec<(Vec<&str>, Vec<&str>)> {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let mut result = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split(" | ").collect();
        let before_pipe: Vec<&str> = parts[0].split_whitespace().collect();
        let after_pipe: Vec<&str> = parts[1].split_whitespace().collect();
        result.push((before_pipe, after_pipe));
    }

    result
}

fn lookup_sorted_vec(map: &HashMap<Vec<char>, i32>, unsorted_vec: Vec<char>) -> Option<i32> {
    let mut sorted_vec = unsorted_vec.clone();
    sorted_vec.sort();
    map.get(&sorted_vec).cloned()
}

fn create_map() -> HashMap<Vec<char>, i32> {
    let mut result = HashMap::new();

    result.insert(vec!['a', 'b', 'c', 'e', 'f', 'g'], 0);
    result.insert(vec!['c', 'f'], 1);
    result.insert(vec!['a', 'c', 'd', 'e', 'g'], 2);
    // add more entries as needed

    result
}

fn parse(values: Vec<&str>) -> HashMap<char, char> {

    let mut result = HashMap::new();

    // let one: Vec<char> = get_based_on_length(values, 2);
    // let seven: Vec<char> = get_based_on_length(values, 3);
    // let four: Vec<char> = get_based_on_length(values, 4);
    // let eight: Vec<char> = get_based_on_length(values, 5);

    // let length_six = values.iter().filter(|x| x.len() == 6);
    // define a variable zero that is a Vec<char> that contains the chars that are in length_six but not in one
    //let zero: Vec<char> = length_six.into_iter().filter(|c| one.contains(&c.chars())).collect();

    result
    

}

fn get_based_on_length(values: Vec<&str>, length: usize) -> Vec<char> {
    values.iter().filter(|x| x.len() == length).map(|x| x.chars().next().unwrap()).collect()
}