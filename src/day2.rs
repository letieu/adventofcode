use std::{fs, path::Path};

#[derive(PartialEq, Debug, Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

#[derive(PartialEq, Debug)]
enum Result {
    Win = 6,
    Lose = 0,
    Draw = 3,
}

fn calculate_score(me: Shape, competitor: Shape) -> i32 {
    let score_by_shape = get_shape_score(&me);

    let result = get_result(me, competitor);
    let score_by_result = result as i32;

    score_by_result + score_by_shape
}

fn get_shape_score(shape: &Shape) -> i32 {
    match shape {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissors => 3,
    }
}

fn get_shape_by_char(c: char) -> Shape {
    match c {
        'A' => Shape::Rock,
        'B' => Shape::Paper,
        'C' => Shape::Scissors,
        _ => panic!("Invalid shape"),
    }
}

fn get_result_by_char(c: char) -> Result {
  match c {
      'X' => Result::Lose,
      'Y' => Result::Draw,
      'Z' => Result::Win,
      _ => panic!("Invalid result"),
  }
}

fn get_shape_by_result(competitor: Shape, result: &Result) -> Shape {
    if *result == Result::Draw {
        return competitor;
    }
    match competitor {
        Shape::Rock => {
            if *result == Result::Win {
                Shape::Paper
            } else {
                Shape::Scissors
            }
        }
        Shape::Paper => {
            if *result == Result::Win {
                Shape::Scissors
            } else {
                Shape::Rock
            }
        }
        Shape::Scissors => {
            if *result == Result::Win {
                Shape::Rock
            } else {
                Shape::Paper
            }
        }
    }
}

fn get_result(me: Shape, competitor: Shape) -> Result {
    if me == competitor {
        return Result::Draw;
    }
    if me == Shape::Rock && competitor == Shape::Scissors {
        return Result::Win;
    }
    if me == Shape::Paper && competitor == Shape::Rock {
        return Result::Win;
    }
    if me == Shape::Scissors && competitor == Shape::Paper {
        return Result::Win;
    }
    Result::Lose
}

#[test]
fn test_win() {
    assert_eq!(Result::Win, get_result(Shape::Rock, Shape::Scissors));
    assert_eq!(Result::Win, get_result(Shape::Paper, Shape::Rock));
    assert_eq!(Result::Win, get_result(Shape::Scissors, Shape::Paper));
}

#[test]
fn test_loose() {
    assert_eq!(Result::Lose, get_result(Shape::Rock, Shape::Paper));
    assert_eq!(Result::Lose, get_result(Shape::Paper, Shape::Scissors));
    assert_eq!(Result::Lose, get_result(Shape::Scissors, Shape::Rock));
}

#[test]
fn test_draw() {
    assert_eq!(Result::Draw, get_result(Shape::Rock, Shape::Rock));
    assert_eq!(Result::Draw, get_result(Shape::Paper, Shape::Paper));
    assert_eq!(Result::Draw, get_result(Shape::Scissors, Shape::Scissors));
}

#[test]
fn test_score() {
    assert_eq!(8, calculate_score(Shape::Paper, Shape::Rock));
    assert_eq!(1, calculate_score(Shape::Rock, Shape::Paper));
    assert_eq!(6, calculate_score(Shape::Scissors, Shape::Scissors));
}

pub fn main() {
    let path = Path::new("input/day2.txt");
    let content = fs::read_to_string(path).expect("Something went wrong reading the file");

    let mut total_score = 0;

    for line in content.lines() {
        let chars = line
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect::<Vec<char>>();

        let competitor = get_shape_by_char(chars[0]);
        let result = get_result_by_char(chars[1]);
        let me = get_shape_by_result(competitor, &result);

        total_score += calculate_score(me, competitor);
    }

    println!("Total score: {}", total_score);
}
