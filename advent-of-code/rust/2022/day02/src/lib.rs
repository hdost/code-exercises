use std::{
    fmt::Debug,
    fs::File,
    io::{BufRead, BufReader},
};

enum HandShape {
    Rock,
    Paper,
    Scissors,
}

impl HandShape {
    fn from<T>(input: &T) -> HandShape
    where
        T: AsRef<str> + Debug + ?Sized,
    {
        match input.as_ref() {
            "A" => HandShape::Rock,
            "X" => HandShape::Rock,
            "B" => HandShape::Paper,
            "Y" => HandShape::Paper,
            "C" => HandShape::Scissors,
            "Z" => HandShape::Scissors,
            a => panic!("Invalid input: {:?}", a),
        }
    }

    /// Determines the value of the `HandShape` chosen
    fn value(&self) -> i64 {
        match self {
            HandShape::Rock => 1,
            HandShape::Paper => 2,
            HandShape::Scissors => 3,
        }
    }

    /// Determines the score you receive for a game
    /// 0 for a loss
    /// 3 for a draw
    /// 6 for a win
    ///
    /// Rules of the game:
    ///
    /// - Rock defeats Scissors
    /// - Scissors defeats Paper
    /// - Paper defeats Rock
    fn score(&self, opponent: HandShape) -> i64 {
        match (self, opponent) {
            (HandShape::Rock, HandShape::Rock) => 3,
            (HandShape::Rock, HandShape::Paper) => 0,
            (HandShape::Rock, HandShape::Scissors) => 6,
            (HandShape::Paper, HandShape::Rock) => 6,
            (HandShape::Paper, HandShape::Paper) => 3,
            (HandShape::Paper, HandShape::Scissors) => 0,
            (HandShape::Scissors, HandShape::Rock) => 0,
            (HandShape::Scissors, HandShape::Paper) => 6,
            (HandShape::Scissors, HandShape::Scissors) => 3,
        }
    }

    /// This determines what shape to chose in when you know what your opponent has chosen and
    /// you want a particular outcome to occur
    fn find_shape(&self, outcome: &RoundOutcome) -> HandShape {
        match (self, outcome) {
            (HandShape::Rock, RoundOutcome::Draw) => HandShape::Rock,
            (HandShape::Rock, RoundOutcome::Win) => HandShape::Paper,
            (HandShape::Rock, RoundOutcome::Loss) => HandShape::Scissors,
            (HandShape::Paper, RoundOutcome::Draw) => HandShape::Paper,
            (HandShape::Paper, RoundOutcome::Win) => HandShape::Scissors,
            (HandShape::Paper, RoundOutcome::Loss) => HandShape::Rock,
            (HandShape::Scissors, RoundOutcome::Draw) => HandShape::Scissors,
            (HandShape::Scissors, RoundOutcome::Win) => HandShape::Rock,
            (HandShape::Scissors, RoundOutcome::Loss) => HandShape::Paper,
        }
    }
}

enum RoundOutcome {
    Win,
    Loss,
    Draw,
}
impl RoundOutcome {
    fn from<T>(input: &T) -> RoundOutcome
    where
        T: AsRef<str> + Debug + ?Sized,
    {
        match input.as_ref() {
            "X" => RoundOutcome::Loss,
            "Y" => RoundOutcome::Draw,
            "Z" => RoundOutcome::Win,
            a => panic!("Invalid input: {:?}", a),
        }
    }

    /// Determines the value of the `HandShape` chosen
    fn value(&self) -> i64 {
        match self {
            RoundOutcome::Loss => 0,
            RoundOutcome::Draw => 3,
            RoundOutcome::Win => 6,
        }
    }
}

#[test]
fn test_first_part_sample_data() {
    let file = File::open("test.txt");
    match file {
        Err(e) => println!("Cannot find file {}", e),
        Ok(file) => {
            assert_eq!(15, process_file(file));
        }
    }
}

#[test]
fn test_first_part_real_data() {
    let file = File::open("input.txt");
    match file {
        Err(e) => println!("Cannot find file {}", e),
        Ok(file) => {
            assert_eq!(10404, process_file(file));
        }
    }
}

#[test]
fn test_second_part_sample_data() {
    let file = File::open("test.txt");
    match file {
        Err(e) => println!("Cannot find file {}", e),
        Ok(file) => {
            assert_eq!(12, process_file_2(file));
        }
    }
}

#[test]
fn test_second_part_real_data() {
    let file = File::open("input.txt");
    match file {
        Err(e) => println!("Cannot find file {}", e),
        Ok(file) => {
            assert_eq!(10334, process_file_2(file));
        }
    }
}

fn process_file(file: File) -> i64 {
    let reader = BufReader::new(file);
    let lines = reader.lines();
    let mut score = 0;
    for line in lines.flatten() {
        let mine = HandShape::from(&line[2..3]);
        let theirs = HandShape::from(&line[0..1]);
        let curr = mine.value() + mine.score(theirs);
        score += curr;
    }

    score
}

fn process_file_2(file: File) -> i64 {
    let reader = BufReader::new(file);
    let lines = reader.lines();
    let mut score = 0;
    for line in lines.flatten() {
        let outcome = RoundOutcome::from(&line[2..3]);
        let theirs = HandShape::from(&line[0..1]);
        let mine = theirs.find_shape(&outcome);
        score += mine.value() + outcome.value();
    }

    score
}
