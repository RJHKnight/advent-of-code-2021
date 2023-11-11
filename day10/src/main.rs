use std::{fs::File, io::{BufReader, BufRead}};

fn main() {
    // load input file
    let input_file_path = "input.txt";
    let input = parse_input_file(input_file_path);

    let mut results = Vec::new();
    for line in input {
        let res = get_completion(line);
        
        // If res is some, add a value to result based on the brace
        if let Some(braces) = res {

            let mut result: i64 = 0;
            for brace in &braces {
                result = result * 5;
                match brace {
                    Brace::Round{is_open: _} => result += 1,
                    Brace::Square{is_open: _} => result += 2,
                    Brace::Curly{is_open: _} => result += 3,
                    Brace::Angle{is_open: _} => result += 4,
                }
            }

            //println!("Result for {:?} {}", braces, result);
            results.push(result);
        }
    }

    // sort result and get the middle value
    results.sort();
    let middle = results.len() / 2;
    println!("Middle value: {}", results[middle]);

}

fn get_completion(line: Vec<Brace>) -> Option<Vec<Brace>> {
    
    // for each brace in line. if it is open, push it to stack. if it is closed, pop from stack and check if it matches
    let mut stack = Vec::new();

    for brace in line {
        if brace.is_open() {
            stack.push(brace);
        } else {
            let last = stack.pop();
            if last.is_none() {
                return None;
            }

            let last = last.unwrap();

            if !last.matches(&brace) {
                return None;
            }
        }
    }

    // Incomplete line  
    // iterate over stack, call matching_brace on each element and add to result
    let mut result = Vec::new();
    for brace in stack.iter().rev() {
        result.push(brace.matching_brace());
    }
    return Some(result);

}

#[derive(Debug)]
enum Brace {
    Round {is_open: bool},
    Square{is_open: bool},
    Curly{is_open: bool},
    Angle{is_open: bool},
}

// is_open method for brace
impl Brace {
    fn is_open(&self) -> bool {
        match self {
            Brace::Round{is_open} => *is_open,
            Brace::Square{is_open} => *is_open,
            Brace::Curly{is_open} => *is_open,
            Brace::Angle{is_open} => *is_open,
        }
    }

    // matches, ignoring is_open
    fn matches(&self, other: &Brace) -> bool {
        match self {
            Brace::Round{is_open: _} => {
                match other {
                    Brace::Round{is_open: _} => true,
                    _ => false
                }
            },
            Brace::Square{is_open: _} => {
                match other {
                    Brace::Square{is_open: _} => true,
                    _ => false
                }
            },
            Brace::Curly{is_open: _} => {
                match other {
                    Brace::Curly{is_open: _} => true,
                    _ => false
                }
            },
            Brace::Angle{is_open: _} => {
                match other {
                    Brace::Angle{is_open: _} => true,
                    _ => false
                }
            },
        }
    }

    fn matching_brace(&self) -> Brace {
        match self {
            Brace::Round{is_open: _} => Brace::Round{is_open: !self.is_open()},
            Brace::Square{is_open: _} => Brace::Square{is_open: !self.is_open()},
            Brace::Curly{is_open: _} => Brace::Curly{is_open: !self.is_open()},
            Brace::Angle{is_open: _} => Brace::Angle{is_open: !self.is_open()},
        }
    }
}

fn parse_input_file(file_path: &str) -> Vec<Vec<Brace>> {
    
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);

    
    let mut result = Vec::new();

    for line in reader.lines() {
        let mut current_vec = Vec::new();

        for c in line.unwrap().chars() {
            let brace = match c {
                '(' => Brace::Round{is_open: true},
                ')' => Brace::Round{is_open: false},
                '[' => Brace::Square{is_open: true},
                ']' => Brace::Square{is_open: false},
                '{' => Brace::Curly{is_open: true},
                '}' => Brace::Curly{is_open: false},
                '<' => Brace::Angle{is_open: true},
                '>' => Brace::Angle{is_open: false},
                _ => todo!("Errr?")
            };


            current_vec.push(brace);
        }

        result.push(current_vec);
    }

    result
}
