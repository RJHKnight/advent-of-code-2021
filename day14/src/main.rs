use std::collections::HashMap;
use std::fs;

fn main() {        
    let (poly_template, pair_rules) = parse_file("input.txt");

    // for step in 0..40 {
    //     react(&mut poly_template, &pair_rules);
    //     //println!("Step {}: {}", step, poly_template.len());
        
    //     // Create a map of char vs count of poly_template
    //     let elemnent_count =poly_template.iter().fold(HashMap::new(), |mut acc, c| {
    //         let count = acc.entry(c).or_insert(0);
    //         *count += 1;
    //         acc
    //     });
    //     // Find the max and min value of char_count
    //     let max = elemnent_count.get(&Element::B).unwrap();
    //     let min = elemnent_count.get(&Element::H).unwrap();

    
    //     println!("Step: {} Max: {}, Min: {}, Difference: {}",step, max, min, max-min);
    // }

    let res = react_with_map(&poly_template, &pair_rules, 40);

    // Loop through res, creating a hashmap of element vs count
    let mut element_count = HashMap::new();
    for (pair, count) in res.iter() {
        let element = pair.0;
        let element_count_entry = element_count.entry(element).or_insert(0);
        *element_count_entry += *count;
        let element = pair.1;
        let element_count_entry = element_count.entry(element).or_insert(0);
        *element_count_entry += *count;
    }

    
    // Loop through element count, if the element is the first or last value in poly_template, add 1 to it and divide by 2, otherwise divide by 2
    for (element, count) in element_count.iter_mut() {  
        
        if element == &poly_template[0] || element == &poly_template[poly_template.len() - 1] {
            *count += 1;
        }
        *count = *count / 2;
    }
    
    println!("Element count: {:?}", element_count);

    // Fing the values of element count that have the max an min value
    let max = element_count.values().max().unwrap();
    let min = element_count.values().min().unwrap();

    println!("Diff = {}", max - min);

}

fn react_with_map(poly_template: &Vec<char>, pair_rules: &HashMap<(char, char), char>, times : i32) -> HashMap<(char, char), i64> {

    // Loop over poly_template 2 entries at a time, creating a map of how many times each pair is found
    let mut pair_count = HashMap::new();
    let mut position = 1;

    while position < poly_template.len() {
        
        let pair = (poly_template[position - 1], poly_template[position]);
        let count = pair_count.entry(pair).or_insert(0);
        *count += 1;
        
        position += 1;
    }

    println!("Initial pair count: {:?}", pair_count);

    for _ in 0..times {
        let mut pair_count_copy = HashMap::new();

        for (pair, count) in pair_count.iter() {
            if pair_rules.contains_key(pair) {
                let result = pair_rules.get(pair).unwrap();
                let pair_one = (*result, pair.1);
                let pair_two = (pair.0, *result);

                let count_one = pair_count_copy.entry(pair_one).or_insert(0);
                *count_one += *count;

                let count_two = pair_count_copy.entry(pair_two).or_insert(0);
                *count_two += *count;
            } 
        }

        pair_count = pair_count_copy;
    }

    pair_count
 
}

fn react(poly_template: &mut Vec<char>, pair_rules: &HashMap<(char, char), char>) {

    let mut position = 1;
    while position < poly_template.len() {

        let pair = (poly_template[position - 1], poly_template[position]);
        if pair_rules.contains_key(&pair) {
            poly_template.insert(position , *pair_rules.get(&pair).unwrap());
            position += 2;
        } else {
            position += 1;
        }
    }
}


fn parse_file(filename: &str) -> (Vec<char>, HashMap<(char, char), char>) {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut lines = contents.lines();

    let poly_template: Vec<char> = lines.next().unwrap().chars().collect();

    let mut pair_rules = HashMap::new();
    for line in lines {
        if line.len() == 0 {
            continue;
        }

        let parts: Vec<&str> = line.split(" -> ").collect();
        let pair: (char, char) = (parts[0].chars().nth(0).unwrap(), parts[0].chars().nth(1).unwrap());
        let result = parts[1].chars().nth(0).unwrap();
        pair_rules.insert(pair, result);
    }

    (poly_template, pair_rules)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_file() {
        let (poly_template, pair_rules) = parse_file("input_sample.txt");

        assert_eq!(poly_template.len(), 4);
        assert_eq!(pair_rules.len(), 16);
        assert_eq!(*pair_rules.get(&('C', 'N')).unwrap(), 'C');
    }
}

