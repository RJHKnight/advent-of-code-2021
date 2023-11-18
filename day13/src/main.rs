use std::collections::HashSet;

fn main() {
    // Read input
    let input = include_str!("../input.txt");
    let (points, folds) = parse_input(input);

    // Initial points
    //println!("Initial points:");
    //print_points(&points);

    // Fold the paper
    let mut folded_points = points.clone();

    let mut count =0;
    for fold in folds {
        count += 1; 
        folded_points = fold_paper(&folded_points, &fold);
        //println!("Fold {:?} ({}):", fold, count);
    }
    
    print_points(&folded_points);
    // Count the points
    println!("Number of points: {}", folded_points.len());


}

// Print points as a grid, with a hash if point is present and a dot if it is missing
fn print_points(points: &HashSet<Point>) {
    let mut min_x = 0;
    let mut max_x = 0;
    let mut min_y = 0;
    let mut max_y = 0;

    for point in points {
        if point.x < min_x {
            min_x = point.x;
        }
        if point.x > max_x {
            max_x = point.x;
        }
        if point.y < min_y {
            min_y = point.y;
        }
        if point.y > max_y {
            max_y = point.y;
        }
    }

    for y in (min_y..max_y+1) {
        for x in min_x..max_x+1 {
            let point = Point { x, y };
            if points.contains(&point) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }

    println!();
}

fn fold_paper(points: &HashSet<Point>, fold: &Fold) -> HashSet<Point> {
    
    let mut folded_points = HashSet::new();

    for point in points {
        if fold.is_vertical {
            if point.y > fold.value {
                let folded_point = Point {
                    x: point.x,
                    y: fold.value - (point.y - fold.value),
                };
                folded_points.insert(folded_point);
            }
            else {
                folded_points.insert(point.clone());
            }
        } else {
            if point.x > fold.value {
                let folded_point = Point {
                    x: fold.value - (point.x - fold.value),
                    y: point.y,
                };
                folded_points.insert(folded_point);
            }
            else {
                folded_points.insert(point.clone());
            }
        }
    }

    folded_points
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Fold {
    is_vertical: bool,
    value: i32,
}

fn parse_input(input: &str) -> (HashSet<Point>, Vec<Fold>) {
    let lines = input.lines();
    let mut points = HashSet::new();
    let mut folds = Vec::new();

    for line in lines {
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() == 2 {
            let point = Point {
                x: parts[0].parse().unwrap(),
                y: parts[1].parse().unwrap(),
            };
            // add point to points
            points.insert(point);

            // Check if the line contains equals sign
        } else if line.contains("=") {

            let parts = line.split_whitespace().collect::<Vec<&str>>();
            let fold_bits: Vec<&str> = parts[2].split("=").collect();

            let fold = Fold {
                is_vertical: fold_bits[0].chars().next().unwrap() == 'y',
                value: fold_bits[1].parse().unwrap(),
            };
            folds.push(fold);
        }
    }

    (points, folds)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = include_str!("../input_sample.txt");
        let (points, folds) = parse_input(input);

        assert_eq!(points.len(), 18);
        assert_eq!(folds.len(), 2);
    }
}