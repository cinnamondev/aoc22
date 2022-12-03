#[aoc_generator(day3)]
pub fn parser(input: &str) -> Vec<Backpack> {
    input
        .split("\n")
        .map(|x| {
            Backpack {
                inv:
                x.chars()
                    .map(|x| {
                        let sep = if x.is_uppercase() {
                            -38
                        } else { // lowercase use -97
                            -96
                        };
                        x as i32 + sep  // now we have letter as priority value instead
                    })
                    .collect::<Vec<i32>>()
            }
        })
        .collect()
}
#[derive(Debug)]
pub struct Backpack {
    inv: Vec<i32> // hacky
}

pub fn find_duplicates(lhs: &[i32], rhs: &[i32]) -> Vec<i32> {
    let mut i = 0;
    let mut dupes: Vec<i32>= Vec::new();
    for &x in lhs {
        if rhs.contains(&x) {
            dupes.push(x)
        }
    }
    dupes.sort_unstable();
    dupes.dedup();
    dupes
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &[Backpack]) -> i32 {
    let mut output_buffer: Vec<i32> = Vec::new();
    for backpack in input {
        let v = &backpack.inv;
        let inv_size = v.len();
        let split = v.split_at(inv_size/2);
        output_buffer.append(
            &mut find_duplicates(split.0, split.1)
        );
    }
    // note to self: dont be dumb idk
    output_buffer.iter().sum()
}
pub fn get_badge(group: (&Backpack, &Backpack, &Backpack)) -> i32 {
    let d1 = find_duplicates(&*group.0.inv, &*group.1.inv);
    find_duplicates(&d1, &*group.2.inv)[0]     // its gross but it works
}

#[aoc(day3,part2)]
pub fn solve_part2(input: &[Backpack]) -> i32 {
    let mut output = 0;
    for i in (0..input.len()).step_by(3) { // 0 3 6 9
        output += get_badge((&input[i], &input[i+1], &input[i+2]));
    }
    output
}