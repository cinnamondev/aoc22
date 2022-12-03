// each row is a "play
// [ENEMY PLAY] [RESPONSE PLAY]

// score is calculated by shape 1=R 2=P 3=S
// Win adds 6 draw adds 3 lose adds 0


use std::str::{FromStr};
use crate::day2::ParserError::{BadPlay, BadShape};
use crate::day2::Shape::{Paper, Rock, Scissors};
use crate::day2::State::{Draw, Lose, Win};
/*
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
*/
#[aoc_generator(day2)]
pub fn parser(input: &str) -> Vec<(u8,u8)> {
    input
        .split("\n")
        .map(|e| {
            // one elf instance
            let c = e.chars().collect::<Vec<char>>();
            (
                map_values(c[0]),
                map_values(c[2])
            )
        })
        .collect()
}
pub fn map_values(c: char) -> u8 {
    match c {
        'A' | 'X' => 1,
        'B' | 'Y' => 2,
        'C' | 'Z' => 3,
        _ => panic!("Unexpected char. check file plz :pleading:"),
    }
}

// ATTEMPT 1
// Attempt 0 was VERY ugly. Uses the same parser,
// though attempt 1has been changed in parts to
// accommodate a different type from parser.
// They definitely arent faster than they were.
// 1 -> 2 -> 3 -> 1 -> ...
// Logic should be like circular list, if we are given a move value,
// the next value will be the winning move,
// previous value is the losing move,
// current value is the drawing move
//

/// Gets the next "move" in the circular list
/// Next value always beats current move
pub fn next_value(n:u8) -> u8 {
    n.rem_euclid(3) + 1 // ( n MOD 3 )+ 1 = Winning move
                            // Take n=3, nMOD3=0 so next is 1, rock beats paper, etc
                            // For n<3, nMOD3=n so next is n+1, this is one way of the circular. neat!
}

/// Gets the previous "move" in the circular list
/// Previous move always loses against current move
pub fn prev_value(n: u8) -> u8 {
    if n == 1 {3} else {n-1}    // prevent out of range value
}

pub fn win_value(play: &(u8,u8)) -> u8 {
    if play.0 != play.1 {
        if prev_value(play.0) == play.1 {
            0   // Lose :) (always win against previous, so player being previous means lose)
        } else {
            6   // Lose (if not previous or same must be next)
        }
    } else {
        3 // DRAW
    }
}

#[aoc(day2,part1,Attempt1)]
pub fn part1_2(input: &[(u8,u8)]) -> u32 {
    input.iter()
        .fold(0, |acc,&x| {
            acc + (x.1 + win_value(&x)) as u32
        })
}

#[aoc(day2,part2, Attempt1)]
pub fn part2_2(input: &[(u8,u8)]) -> u32 {
    // x.1 in this case will be the "move" to play
    input.iter()
        .map(|&p| {
            let next_move = match p.1 {
                1 => prev_value(p.0),
                3 => next_value(p.0),
                _ => p.0 // 2 and other is draw to make this nice
            };
            next_move + win_value(&(p.0, next_move))
        } as u32)
        .sum()
}


// Attempt 0 (very bad code written at 5am, please never read this)
#[derive(Debug, Copy, Clone)]
pub enum ParserError {
    BadShape,
    BadPlay,
    BadInput,
}
#[derive(Debug, Copy, Clone)]                             // stop trying to be fancy :(
pub enum Shape {
    Rock = 1,       // Precedence Scissors -> Paper -> Rock ->
    Paper = 2,
    Scissors = 3,
    wack = -1,
}
impl Shape {
    pub fn from_int(i: i32) -> Shape {
        match i {
            1 => Rock,
            2 => Paper,
            3 => Scissors,
            _ => panic!("")
        }
    }
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
impl Shape {
    pub fn from_num(n:u8) -> Shape {
        match n {
            1 => Rock,
            2 => Paper,
            3 => Scissors,
            _ => panic!()
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
    pub fn get_score(&self) -> u32 {
        match self {
            Win(s) => 6+(*s as u32),
            Draw(s) => 3+(*s as u32),
            Lose(s) => *s as u32,
        }
    }
}
#[derive(Debug, Copy, Clone)]
pub struct Play {
    enemy: Shape,
    player: Shape,
}
impl Play {
    pub fn from_value(e: u8, p: u8) -> Self {
        Self {
            enemy: Shape::from_num(e),
            player: Shape::from_num(p),
        }
    }
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

#[aoc(day2, part1, Attempt0)]
pub fn solve_part1(input: &[(u8,u8)]) -> u32 {
    input.iter()
        .fold(
            0, |acc, x| {
                let play = Play::from_value(x.0,x.1);
                acc + play
                    .get_state()
                    .get_score()
            })
}


pub fn predict_move(outcome: Shape, enemy: Shape) -> Shape {
    match outcome {
        // yes it looks gross but it saves refactoring any of part1.
        Scissors => Shape::from_int((enemy as i32).rem_euclid(3)+1),
        Paper => enemy,
        Rock => match enemy {    // was really hoping there would be a nice neat maths thing here...
            Rock => Scissors,
            Paper => Rock,
            Scissors => Paper,
            _ => panic!("")
        },
        _ => panic!("")
    }
}
// part 2 is gonna be funky
#[aoc(day2, part2, Attempt0)]
pub fn solve_part2(input: &[(u8,u8)]) -> u32 {
    let play: Vec<Play> =input.iter()
        .map(|x| {
            let play = Play::from_value(x.0,x.1);
            // oh no.. here it comes
            Play {
                enemy: play.enemy,
                player: predict_move(play.player, play.enemy),
            }
        }).collect();
    // so it can remain in the spirit of the original attempt
    let jank = play.iter().map(|p| (p.enemy as u8, p.player as u8)).collect::<Vec<(u8,u8)>>();
    solve_part1(&jank) // Total up
}