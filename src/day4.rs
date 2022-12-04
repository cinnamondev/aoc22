use std::fmt::format;

const MSB: u128 = 2_u128.pow(127);

#[aoc_generator(day4)]
pub fn parser(input: &str) -> Vec<((u8,u8),(u8,u8))> {
    input.split("\n")
        .map(|s| {
            let range = s.split(",")
                .map(|s| {
                    let num = s.split("-")
                        .map(|s| {
                            s.parse::<u8>().expect("Uh oh")
                        }).take(2)
                        .collect::<Vec<u8>>();
                    (num[0],num[1])
                })
                .collect::<Vec<(u8,u8)>>();
            (range[0],range[1])
                // binary representation
                // if 9 slots, we need 9 bits, one for each bit
                // ie: 2-4 = 011100000
                //     3-5 = 001110000
                // We can find complete overlaps with a bitwise AND
                // 2-4 AND 3-5 = 001100000
        })
        .collect()
}

pub fn dep_bit_representation(range: (u8, u8)) -> u128 { // big binary!!
    let mut to_set = 0b000000000;
    for i in range.0..=range.1 {
        to_set += MSB >> i-1;   // set each bit representing a
                                // slot high for that range
    }
    to_set
}
pub fn bit_range(range:(u8,u8)) -> u128 {
    let (start,stop) = range;
    (u128::MAX >> (128-stop+start-1)) << (128-stop) // Binary magic
}

#[aoc(day4, part1, Binary)]
pub fn solve_part1(input: &[((u8,u8),(u8,u8))]) -> u32 {
    input.iter()
        .fold(0, |acc, &range| {
            let (a,b) = (bit_range(range.0),
                         bit_range(range.1));
            let res = a&b;
            if res==a || res == b {
                acc +1
            } else {
                acc
            }
        })
}

#[aoc(day4, part2, Binary)]
pub fn solve_part2(input: &[((u8,u8),(u8,u8))]) -> u32 {
    input.iter()
        .fold(0, |acc, &range| {
            let (a,b) = (bit_range(range.0),
                         bit_range(range.1));
            match a & b {
                0 => acc,
                _ => acc+1
            }
        })
}
// Maybe faster? lets find out :)
#[aoc(day4, part1, Logic)]
pub fn logic_solve_part1(input: &[((u8,u8),(u8,u8))]) -> u32 {
    input.iter()
        .fold(0, |acc, &group| {
            let (a,b) = group;
            if (a.0 >= b.0 && a.1 <= b.1) || (b.0 >= a.0 && b.1 <= a.1) {
                acc+1
            } else {
                acc
            }
        })
}
#[aoc(day4, part2, Logic)]
pub fn logic_solve_part2(input: &[((u8,u8),(u8,u8))]) -> u32 {
    input.iter()
        .fold(0, |acc, &group| {
            let (a, b) = group;
            if (a.0 >= b.0 && a.0 <= b.1) || (b.0 >= a.0 && b.0 <= a.1) {
                acc + 1
            } else {
                acc
            }
        })
}