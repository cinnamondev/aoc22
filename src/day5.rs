use std::ops::Index;

#[aoc_generator(day5)]
pub fn parser(input: &str) -> Inventory {
    let parts = input.split("\n\n").collect::<Vec<&str>>(); // split 2 sections of the puzzle
    // not good
    //let width = 9;
    /*         parts[0].lines()
            .last()
            .expect("a")
            .split_whitespace()
            .count()*/
    //let max_height = 9*9; /*parts[0].lines().count();*/
    let mut stacks: Vec<Vec<char>> = Vec::new();
    let _: Vec<i32> = parts[0].lines()
        .rev()// reverse so in addition order
        .skip(1)// skip index line
        //.take(1)
        .map(|l| {
            l.chars()
                .skip(1)// first 2 arent letters, consume!
                .step_by(4)// get only the letters
                .fold(0, |acc, x| {
                    //dbg!(x);
                    //dbg!(x as u32);
                    //dbg!(stacks.len()as i32-1, acc);
                    if x!=' ' {
                        if acc > (stacks.len() as i32-1) {
                            //dbg!("woah");
                            stacks.push(vec![x]);
                        } else {
                            stacks[acc as usize].push(x);
                        }
                        //dbg!(x);
                    }
                    acc + 1
                })
        })
        .collect();
    //dbg!(&stacks);
    let moves = parts[1].lines()// for each line
        .map(|x| {
            let v = x.split(" ")
                .filter_map(|s| s.parse::<usize>().ok())
                .collect::<Vec<usize>>();
            (v[0],v[1],v[2])
        })
        .collect();
    //dbg!(&moves); //gtg
    Inventory {
        stacks,
        moves
    }
}

#[aoc(day5,part1)]
pub fn solve_part1(input: &Inventory) -> String {
    let mut stacks = input.stacks.clone();
    let moves = &input.moves;
    for m in moves {
        for _ in 1..=m.0 {
            //dbg!(m,&origin,&target);
            let remove = stacks[m.1-1].pop().expect("aa");
            stacks[m.2-1].push(remove);
        }
    }
    let mut output = String::new();
    for mut stack in stacks {
        // hack
        let item = stack.iter().last();
        output.push(*item.expect("lol"));
    }
    output

}

#[aoc(day5,part2)]
pub fn solve_part2(input: &Inventory) -> String {
    let mut stacks = input.stacks.clone();
    let moves = &input.moves;
    for m in moves {
        let drain_index = stacks[m.1-1].len()-m.0;
        let mut crates = stacks[m.1-1].split_off(drain_index);
        stacks[m.2-1].append(&mut crates);
    }
    let mut output = String::new();
    for mut stack in stacks {
        // hack
        let item = stack.iter().last();
        output.push(*item.expect("lol"));
    }
    output
}

    #[derive(Clone, Debug)]
pub struct Inventory { // Make `cargo-aoc` happy.
    stacks: Vec<Vec<char>>,
    moves: Vec<(usize,usize,usize)>
}