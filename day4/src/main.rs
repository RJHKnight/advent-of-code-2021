use std::{fs::File, io::{BufReader, BufRead}};
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};


fn main() {
    // parse input file
    let (mut nums, mut boards) = parse_input_file("input.txt").unwrap();

    while boards.len() > 0 {
        
        let winner = play_bingo(&nums, &mut boards);
        match winner {
            Some(winner) => {
                
                if boards.len() == 1 {
                    println!("Last Winner is board #{}!", winner.0 + 1);
                    (boards[winner.0].print_board());
                    println!("Score of last winner is {}", 
                        boards[winner.0].sum_unchecked() * nums[winner.1]);
                    break;
                }
                else {
                    boards.remove(winner.0);  
                    // truncate nums based on winner.2
                    nums = nums.split_off(winner.1);
                }

            },
            None => {
                println!("No winner!");
            }
        }
    }

}

fn play_bingo(numbers: &Vec<u16>, boards: &mut Vec<BingoBoard>) -> Option<(usize, usize)> {
    for num_count in 0..numbers.len() {
        let num = numbers[num_count];
        for (i, board) in boards.iter_mut().enumerate() {
            board.set_checked(num);
            if board.is_bingo() {
                return Some((i,num_count));
            }
        }
    }
    None
}


fn parse_input_file(filename: &str) -> Result<(Vec<u16>, Vec<BingoBoard>), std::io::Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut lines = reader.lines();
    let first_line = lines.next().unwrap()?;
    let nums: Vec<u16> = first_line
        .split(',')
        .map(|s| s.trim().parse().unwrap())
        .collect();

    let mut boards = Vec::new();
  
    let mut current_board = [[0; 5]; 5];
    let mut board_index = 0;
    let mut is_new_board = true;
    
    while let Some(line) = lines.next() {
        let line = line?;
        if line.trim().is_empty() {
            if is_new_board {
                continue;
            } else {
                boards.push(BingoBoard::new(current_board));
                current_board = [[0; 5]; 5];
                board_index = 0;
                is_new_board = true;
            }
        } else {
            let board_row: [u16; 5] = line
                .split_whitespace()
                .map(|s| s.trim().parse().unwrap())
                .collect::<Vec<u16>>()
                .try_into()
                .unwrap();
            current_board[board_index] = board_row;
            board_index += 1;
            is_new_board = false;
        }
    }
    
    if !is_new_board {
        boards.push(BingoBoard::new(current_board));
    }
    Ok((nums, boards))
}

#[derive(Clone)]
struct BingoBoard {
    board: [[u16; 5]; 5],
    checked: [[bool; 5]; 5],
}

impl BingoBoard {

    fn new(board: [[u16; 5]; 5]) -> Self {
        Self {
            board,
            checked: [[false; 5]; 5],
        }
    }

    fn set_checked(&mut self, num: u16) {
        for i in 0..5 {
            for j in 0..5 {
                if self.board[i][j] == num {
                    self.checked[i][j] = true;
                    return;
                }
            }
        }
    }

    fn is_bingo(&self) -> bool {
        // Check rows
        for i in 0..5 {
            if self.checked[i].iter().all(|&x| x) {
                return true;
            }
        }

        // Check columns
        for j in 0..5 {
            if (0..5).all(|i| self.checked[i][j]) {
                return true;
            }
        }

        false
    }

    fn sum_unchecked(&self) -> u16 {
        let mut sum = 0;
        for i in 0..5 {
            for j in 0..5 {
                if !self.checked[i][j] {
                    sum += self.board[i][j];
                }
            }
        }
        sum
    }

    fn print_board(&self) {
        let mut stdout = StandardStream::stdout(ColorChoice::Always);

        for i in 0..5 {
            for j in 0..5 {
                let num = self.board[i][j];
                let color = if self.checked[i][j] {
                    Color::Red
                } else {
                    Color::Black
                };
                let mut color_spec = ColorSpec::new();
                color_spec.set_fg(Some(color)).set_bold(true);
                write!(&mut stdout, "{:>2} ", num).unwrap();
                stdout.set_color(&color_spec).unwrap();
                write!(&mut stdout, "■ ").unwrap();
                stdout.reset().unwrap();
            }
            writeln!(&mut stdout).unwrap();
        }
    }
}