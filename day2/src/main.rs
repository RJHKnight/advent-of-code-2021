use std::{fs::File, io::{BufReader, BufRead}, f32::consts::E};

// Load the input csv file and parse the instructions for moving the submarine.
// After moving the submarine, print the final position of the submarine.
fn main() {
    let file = File::open("input.csv").unwrap();
    let reader = BufReader::new(file);
    let mut position = Position {
        distance: 0,
        depth: 0,
        aim: 0,
    };
    
    for line in reader.lines() {
        if let Some(movement) = parse_submarine_movement(&line.unwrap()) {
            position = position.move_submarine(movement).unwrap();
        }
    }
    println!("Final position: distance={}, depth={}, multiple={}", 
        position.distance, 
        position.depth,
        position.distance * position.depth);
}

#[derive(Debug, PartialEq)]
enum SubmarineMovement {
    Forward(i32),
    Up(i32),
    Down(i32),
}

fn parse_submarine_movement(input: &str) -> Option<SubmarineMovement> {
    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.len() != 2 {
        return None;
    }
    let amount = parts[1].parse::<i32>().ok()?;
    match parts[0].to_lowercase().as_str() {
        "forward" => Some(SubmarineMovement::Forward(amount)),
        "up" => Some(SubmarineMovement::Up(amount)),
        "down" => Some(SubmarineMovement::Down(amount)),
        _ => None,
    }
}

#[derive(Debug, PartialEq)]
struct Position {
    distance: i32,
    depth: i32,
    aim: i32,
}

impl Position {
    fn move_submarine(&self, movement: SubmarineMovement) -> Result<Position, String> {
        match movement {
            SubmarineMovement::Forward(amount) => {
                let new_depth = self.depth + (self.aim * amount);
                if new_depth >= 0 {
                    Ok(Position {
                        distance: self.distance + amount,
                        depth: new_depth,
                        aim: self.aim,
                    })
                } else {
                    Err(String::from("Submarine depth cannot be negative"))
                } 

            },
            SubmarineMovement::Up(amount) => Ok(Position {
                distance: self.distance,
                depth: self.depth,
                aim: self.aim - amount as i32,
            }),
            SubmarineMovement::Down(amount) => Ok(Position {
                distance: self.distance,
                depth: self.depth,
                aim: self.aim + amount as i32,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_submarine_movement() {
        assert_eq!(parse_submarine_movement("forward 10"), Some(SubmarineMovement::Forward(10)));
        assert_eq!(parse_submarine_movement("up 10"), Some(SubmarineMovement::Up(10)));
        assert_eq!(parse_submarine_movement("down 10"), Some(SubmarineMovement::Down(10)));
        assert_eq!(parse_submarine_movement("left 10"), None);
    }

    // #[test]
    // fn test_move_submarine() {
    //     let position = Position {
    //         distance: 0,
    //         depth: 0,
    //     };
    //     assert_eq!(position.move_submarine(SubmarineMovement::Forward(10)).unwrap(), Position {
    //         distance: 10,
    //         depth: 0,
    //     });

    //     // Test invalid submarine movement
    //     assert_eq!(position.move_submarine(SubmarineMovement::Up(10)), 
    //         Err(String::from("Submarine depth cannot be negative")));

    //     assert_eq!(position.move_submarine(SubmarineMovement::Down(10)).unwrap(), Position {
    //         distance: 0,
    //         depth: 10,
    //     });
    // }

    #[test]
    fn test_move_submarine() {
        let position = Position {
            distance: 0,
            depth: 0,
            aim: 0,
        };
        
        let position = position.move_submarine(SubmarineMovement::Forward(5)).unwrap();
        assert_eq!(position, Position {
            distance: 5,
            depth: 0,
            aim: 0,
        });

        let position = position.move_submarine(SubmarineMovement::Down(5)).unwrap();
        let position = position.move_submarine(SubmarineMovement::Forward(8)).unwrap();
        let position = position.move_submarine(SubmarineMovement::Up(3)).unwrap();
        let position = position.move_submarine(SubmarineMovement::Down(8)).unwrap();
        let position = position.move_submarine(SubmarineMovement::Forward(2)).unwrap();
        assert_eq!(position.depth, 60);
        assert_eq!(position.distance, 15);
    }
}