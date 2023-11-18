use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::fmt::{self, Display};

fn main() {
    let (room_map, start_room) = parse_map("input.txt");

    let mut room_count: HashMap<Room, i32> = HashMap::new();
    
    let mut all_paths: Vec<Vec<Room>> = Vec::new();
    if let Some(start_room) = start_room {
        dfs(&start_room, &mut Vec::new(), &room_map, &mut all_paths, &mut room_count);
    }
    
    // for path in &all_paths {
    //     print_path(&path);
    // }

    println!("Total paths: {}", all_paths.len());
}

// fn print_path(path: &Vec<Room>) {
//     let path_str: Vec<String> = path.iter().map(|room| room.to_string()).collect();
//     println!("{}", path_str.join(" -> "));
// }

fn dfs(
    current_room: &Room,
    current_path: &mut Vec<Room>,
    room_map: &HashMap<Room, Vec<Room>>,
    all_paths: &mut Vec<Vec<Room>>,
    room_count: &mut HashMap<Room, i32>,
) {
    current_path.push(current_room.clone());

    if current_room.is_end {
        all_paths.push(current_path.clone());
    } else {
        if let Some(connected_rooms) = room_map.get(current_room) {
            for connected_room in connected_rooms {

                // Check if any small caves have been visited twice
                let any_twice = room_count.iter().map(|(room, count)| {
                    if *count >= 2 && !room.is_capital {
                        return true;
                    }
                    false
                }).any(|x| x);

                let max_visits = if any_twice { 1 } else { 2 };

                // Check the number of times we have been in this room
                let mut cloned_room_count = room_count.clone();
                let count = cloned_room_count.entry(connected_room.clone()).or_insert(0);
                let is_start = connected_room.is_start;
                let can_enter_room = (connected_room.is_capital || *count < max_visits) & !is_start;


                if can_enter_room {

                    // Increment the count
                    *count += 1;

                    //println!("In Room {} and going to {}", current_room, connected_room);
                    dfs(connected_room, current_path, room_map, all_paths, &mut cloned_room_count);
                }
            }
        }
    }

    current_path.pop();
    
}
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Room {
    name: String,
    is_start: bool,
    is_end: bool,
    is_capital: bool,
}


impl Display for Room {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

fn parse_map(file_name: &str) -> (HashMap<Room, Vec<Room>>, Option<Room>) {
    let mut room_map: HashMap<Room, Vec<Room>> = HashMap::new();
    let input_file = File::open(&Path::new(file_name)).unwrap();
    let mut start_room = None;

    for line in io::BufReader::new(input_file).lines() {
        let line = line.unwrap();
        let rooms: Vec<&str> = line.split('-').collect();
        let room1 = Room {
            name: rooms[0].to_string(),
            is_start: rooms[0].starts_with("start"),
            is_end: rooms[0].ends_with("end"),
            is_capital: rooms[0].chars().next().unwrap().is_uppercase(),
        };

        if room1.is_start {
            start_room = Some(room1.clone());
        }

        let room2 = Room {
            name: rooms[1].to_string(),
            is_start: rooms[1].starts_with("start"),
            is_end: rooms[1].ends_with("end"),
            is_capital: rooms[1].chars().next().unwrap().is_uppercase(),
        };
        room_map.entry(room1.clone()).or_insert_with(Vec::new).push(room2.clone());
        room_map.entry(room2).or_insert_with(Vec::new).push(room1);

    }

    (room_map, start_room)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_map() {
        
        let room_map = parse_map("input_sample.txt").0;

        let start = Room {
            name: "start".to_string(),
            is_start: true,
            is_end: false,
            is_capital: false,
        };
        let A = Room {
            name: "A".to_string(),
            is_start: false,
            is_end: false,
            is_capital: true,
        };
        let end = Room {
            name: "end".to_string(),
            is_start: false,
            is_end: true,
            is_capital: false,
        };

        assert_eq!(room_map[&start].contains(&A), true);
        assert_eq!(room_map[&A].contains(&end), true);
    }
}