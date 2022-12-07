#[aoc(day6,part1)]
pub fn solve_part1(input: &str) -> usize {
    let v: Vec<_> = input.chars().collect();
    let mut o = 0;
    for i in (3..v.len()) {
        let rolling = v[i-3..=i].to_vec();
        let mut rolling_dedup = rolling.clone();
        rolling_dedup.sort_unstable(); // not really cheap
        rolling_dedup.dedup();
        if rolling.len() == rolling_dedup.len() {
            dbg!(rolling.len(),rolling_dedup.len());o=i+1;break;
        }
    }
    o
}

#[aoc(day6,part2)]
pub fn solve_part2(input: &str) -> usize {
    let v: Vec<_> = input.chars().collect();
    let mut o = 0;
    for i in (13..v.len()) {
        let rolling = v[i-13..=i].to_vec();
        let mut rolling_dedup = rolling.clone();
        rolling_dedup.sort_unstable(); // not really cheap
        rolling_dedup.dedup();
        if rolling.len() == rolling_dedup.len() {
            dbg!(rolling.len(),rolling_dedup.len());o=i+1;break;
        }
    }
    o
}