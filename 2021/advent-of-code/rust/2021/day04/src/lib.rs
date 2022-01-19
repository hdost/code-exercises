use std::{
    fmt,
    fs::File,
    io::{BufRead, BufReader, Error},
};

#[test]
#[ignore]
fn test_first_part_sample_data() {
    let file = File::open("test.txt");
    match file {
        Err(e) => println!("Cannot find file {}", e),
        Ok(file) => process_file(file),
    }
}

#[test]
#[ignore]
fn test_first_part_real_data() {
    let file = File::open("input.txt");
    match file {
        Err(e) => println!("Cannot find file {}", e),
        Ok(file) => process_file(file),
    }
}

#[test]
#[ignore]
fn test_second_part_sample_data() {
    let file = File::open("test.txt");
    match file {
        Err(e) => println!("Cannot find file {}", e),
        Ok(file) => process_file_2(file),
    }
}

#[test]
fn test_second_part_real_data() {
    let file = File::open("input.txt");
    match file {
        Err(e) => println!("Cannot find file {}", e),
        Ok(file) => process_file_2(file),
    }
}

#[derive(Debug, Default, Clone)]
struct Position {
    pub value: i32,
    pub marked: bool,
}

impl fmt::Display for Position {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        if self.marked {
            write!(f, "({})", self.value)
        } else {
            write!(f, "{}", self.value)
        }
    }
}

#[derive(Debug, Default, Clone)]
struct Board {
    pub squares: Vec<Vec<Position>>,
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for col in self.squares.iter() {
            for v in col.iter() {
                write!(f, "{}\t", v)?;
            }

            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl Board {
    fn make() -> Self {
        Board {
            squares: vec![vec![Position::default(); 5]; 5],
        }
    }
    fn from_vec(input: &Vec<String>) -> Self {
        let mut board = Board::make();
        for (i, line) in input.iter().enumerate() {
            let nums: Vec<&str> = line.split_whitespace().collect();
            for (j, n) in nums.iter().enumerate() {
                if let Ok(n) = n.to_string().trim().parse::<i32>() {
                    board.squares[i][j].value = n;
                };
            }
        }
        board
    }
    fn winner_sum(&self) -> i32 {
        let mut output = 0;
        for col in self.squares.iter() {
            for p in col {
                if !p.marked {
                    output += p.value;
                }
            }
        }
        output
    }
    fn mark(&mut self, num: i32) {
        for col in self.squares.iter_mut() {
            for s in col {
                if s.value == num {
                    s.marked = true;
                }
            }
        }
    }
    fn winner(&self) -> bool {
        let mut row_sums = vec![0; 5];
        for col in self.squares.iter() {
            let mut col_sum = 0;
            for (i, p) in col.iter().enumerate() {
                if p.marked {
                    row_sums[i] += 1;
                    col_sum += 1;
                }
            }
            if col_sum == 5 {
                return true;
            }
        }
        for s in row_sums {
            if s == 5 {
                return true;
            }
        }
        false
    }
}

fn make_num_list(line: Option<Result<String, Error>>) -> Vec<i32> {
    let mut nums_list = Vec::new();
    match line {
        Some(line) => match line {
            Ok(line) => {
                println!("Some: {}", line);
                let nums: Vec<&str> = line.split(',').collect();
                for n in nums {
                    if let Ok(n) = n.to_string().trim().parse::<i32>() {
                        nums_list.insert(0, n);
                    };
                }
            }
            Err(e) => panic!("File Err: {}", e),
        },
        None => {
            panic!("Empty File!");
        }
    }
    nums_list
}

pub fn process_file(file: File) {
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let mut board_list = Vec::new();
    // Populate the Num List
    let mut nums_list = make_num_list(lines.next());
    // Build Boards
    let mut curr_board = Vec::new();
    for line in lines.into_iter() {
        if let Ok(line) = &line {
            if line == "" {
                if curr_board.len() > 0 {
                    board_list.push(Board::from_vec(&curr_board));
                    curr_board = Vec::new();
                }
            } else {
                curr_board.push(line.to_string());
            }
            println!("{} {}", line == "", line);
        }
        if curr_board.len() == 5 {
            board_list.push(Board::from_vec(&curr_board));
        }
    }
    // Play The Game
    let mut curr_num = 0;
    let mut winner = 0;
    while nums_list.len() > 0 && winner == 0 {
        curr_num = nums_list.pop().unwrap();
        println!("Curr Num: {} ", curr_num);
        for b in board_list.iter_mut() {
            // Mark the number
            b.mark(curr_num);
            // Check if winner
            println!("{}", b);
            if b.winner() {
                winner = b.winner_sum();
                println!("Winner! {}", winner);
                break;
            }
        }
    }
    println!("Output: {}", curr_num * winner);
}

pub fn process_file_2(file: File) {
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let mut board_list = Vec::new();
    // Populate the Num List
    let mut nums_list = make_num_list(lines.next());
    // Build Boards
    let mut curr_board = Vec::new();
    for line in lines.into_iter() {
        if let Ok(line) = &line {
            if line == "" {
                if curr_board.len() > 0 {
                    board_list.push(Board::from_vec(&curr_board));
                    curr_board = Vec::new();
                }
            } else {
                curr_board.push(line.to_string());
            }
            println!("{} {}", line == "", line);
        }
        if curr_board.len() == 5 {
            board_list.push(Board::from_vec(&curr_board));
        }
    }
    // Play The Game
    let mut curr_num = 0;
    let mut winner = 0;
    while nums_list.len() > 0 && board_list.len() > 0 {
        curr_num = nums_list.pop().unwrap();
        println!("Curr Num: {} ", curr_num);
        let mut curr_winners = Vec::new();
        for (i,b) in board_list.iter_mut().enumerate() {
            // Mark the number
            b.mark(curr_num);
            // Check if winner
            if b.winner() {
            println!("{}", b);
                winner = b.winner_sum();
                curr_winners.push(i);
                println!("Winner! {}", winner);
            }
        }
        board_list.retain(|e|{
            !e.winner()
        });
    }
    println!("Output: {}", curr_num * winner);
}
