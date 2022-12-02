// each row is a "play
// [ENEMY PLAY] [RESPONSE PLAY]

// score is calculated by shape 1=R 2=P 3=S
// Win adds 6 draw adds 3 lose adds 0


use std::fmt::Error;
use std::str::{Chars, FromStr};
use crate::day2;
use crate::day2::ParserError::{BadPlay, BadShape};
use crate::day2::Shape::{Paper, Rock, Scissors};
use crate::day2::State::{Draw, Lose, Win};

#[aoc_generator(day2)]
pub fn parser(input: &str) -> Vec<Play> {
    input        // "1000\r\n2000\r\n\r\n3000"
        .split("\n").collect::<Vec<&str>>().iter()
        .map(|e| {
            // one elf instance
            Play::from_str(e).expect("crap")
        })
        .collect()
}

#[derive(Debug, Copy, Clone)]
pub enum ParserError {
    BadShape,
    BadPlay,
    BadInput,
}
#[derive(Debug, Copy, Clone)]
pub enum Shape {
    Rock = 1,       // Precedence Scissors -> Paper -> Rock ->
    Paper = 2,
    Scissors = 3,
}
impl FromStr for Shape {
    type Err = ParserError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X"=> Ok(Rock),
            "B" | "Y"  => Ok(Paper),
            "C" | "Z" => Ok(Scissors),
            _ => Err(BadShape)
        }
    }
}
#[derive(Debug, Copy, Clone)]
pub enum State {
    Win(Shape),
    Draw(Shape),
    Lose(Shape),
}
impl State {
    pub fn get_score(&self) -> i32 {
        match self {
            Win(s) => 6+(*s as i32),
            Draw(s) => 3+(*s as i32),
            Lose(s) => *s as i32,
            _ => panic!("what"),
        }
    }
}
#[derive(Debug, Copy, Clone)]
pub struct Play {
    enemy: Shape,
    player: Shape,
}
impl Play {
    pub fn get_state(&self) -> State {
        let winner = (*&self.enemy as i32- *&self.player as i32).rem_euclid(3);
        match winner {
            0 => Draw(self.player),
            1 => Lose(self.player),
            2 => Win(self.player),
            _ => panic!("what")
        }
    }
}
impl FromStr for Play {
    type Err = ParserError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut c = s.chars().filter(|x| *x!=' ');
        let c1= if let Some(c) = c.next() {c.to_string()} else {return Err(BadPlay)};
        let c2= if let Some(c) = c.next() {c.to_string()} else {return Err(BadPlay)};
        Ok(
        Self {
            enemy: Shape::from_str(&*c1)?,
            player: Shape::from_str(&*c2)?
        }
        )
    }
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[Play]) -> i32 {
    input.iter()
        .fold(
            0, |acc, x| {
                acc + x
                    .get_state()
                    .get_score()
            })
}