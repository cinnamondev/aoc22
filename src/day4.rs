use std::fmt::format;

const MSB: u128 = 2_u128.pow(127);

#[aoc_generator(day4)]
pub fn parser(input: &str) -> Vec<((u8,u8),(u8,u8))> {
    input.split("\n")
        .map(|s| {
            let range = s.split(",")
                .map(|s| {
                    let n = s.split("-").collect::<Vec<&str>>();
                    (
                        n[0].parse::<u8>().expect("er"),
                        n[1].parse::<u8>().expect("err")
                    )
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

pub fn bit_representation(range: (u8, u8)) -> u128 { // big binary!!
    let mut to_set = 0b000000000;
    for i in range.0..=range.1 {
        to_set += MSB >> i-1;   // set each bit representing a
                                // slot high for that range
    }
    to_set
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &[((u8,u8),(u8,u8))]) -> u32 {
    let mut counter = 0;
    for group in input { // groups of two elves
        let elf_a = bit_representation(group.0);
        let elf_b = bit_representation(group.1);
        let overlap = elf_a & elf_b;
        if overlap == elf_a || overlap == elf_b {
            counter+=1;
        }
    }
    counter
}
#[aoc(day4, part2)]
pub fn solve_part2(input: &[((u8,u8),(u8,u8))]) -> u32 {
    let mut counter = 0;
    for group in input { // groups of two elves
        let elf_a = bit_representation(group.0);
        let elf_b = bit_representation(group.1);
        let overlap = elf_a & elf_b;
        if overlap != 0 {
            counter+=1;
        }
    }
    counter
}