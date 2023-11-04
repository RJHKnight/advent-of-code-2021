use std::collections::VecDeque;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    println!("Simple count: {}", count(&input));
    println!("Moving average count: {}", count_moving_average(&input));
}   

fn count(input: &str) -> i32 {
    
    let mut count = 0;
    let mut prev = None;
        
    for line in input.lines() {
        let num = line.parse::<i32>().unwrap();
        
        if prev.is_some() && num > prev.unwrap() {
            count += 1;
        }

        prev = Some(num);
    }   

    count
}

fn count_moving_average(input: &str) -> i32 {
    
    let mut count = 0;
    let mut queue = VecDeque::new();
        
    for line in input.lines() {
        let num = line.parse::<i32>().unwrap();
        
        if queue.len() == 3 {

            // Previous average
            let prev_avg = average_queue(&queue);

            queue.pop_front();
            queue.push_back(num as f32);

            // New Average
            let new_avg = average_queue(&queue);

            if new_avg > prev_avg {
                count += 1;
            }

        }
        else {
            queue.push_back(num as f32);
        }

    }   

    count
}

fn average_queue(queue: &VecDeque<f32>) -> f32 {
    let sum: f32 = queue.iter().sum();
    sum / queue.len() as f32
}

// write a test case for the above code
// Path: src/main.rs
#[cfg(test)]
mod tests {
    use crate::{count, count_moving_average};

    #[test]
    fn test_count() {
        let input = "5\n3\n6\n7";
        let count = count(input);
        assert_eq!(count, 2);
    }

    // test the moving average count 
    #[test]
    fn test_count_moving_average() {
        
        let input = "199\n200\n208\n210\n200\n207\n240\n269\n260\n263";

        let count = count_moving_average(input);
        assert_eq!(count, 5);
    }
}

