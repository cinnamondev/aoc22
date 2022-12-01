//desc each line represents a place in inventory (arr), and each empty new line represents new elf
// [[1000,3000,...],[],[],...,[]]
/// 1000
/// 1000
///
/// 2000
/// 1000
//  3000
//
//  4000

// Find elf with MOST calories in inventory


#[aoc_generator(day1)]
pub fn parser(input: &str) -> Vec<Vec<i32>> {
    input        // "1000\r\n2000\r\n\r\n3000"
        .split("\n\n").collect::<Vec<&str>>().iter()
        .map(|e| {
            // one elf instance
            e
                .split("\n")
                .map(|c| c.parse::<i32>().unwrap_or(-1))
                .collect()
        })
        .collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &Vec<Vec<i32>>) -> i32 {
    input
        .iter()
        .map(|vec| {
            vec.iter().sum()
        })
        .max()
        .unwrap_or(-1)
}

#[aoc(day1,part2)]
pub fn solve_part2(input: &Vec<Vec<i32>>) -> String {
    // get top 3 elves. max algo
    // make algo that is played 3 times, once max is found remove
    let mut sum: Vec<i32> = input
        .iter()
        .map(|v| {
            v.iter().sum()
        })
        .collect();// get sum of inventory
    let mut ret : Vec<i32> = Vec::with_capacity(3);
    for _ in 0..=2 {
        let max: i32 = *sum.iter().max().unwrap_or(&-1);
        let i_max = sum
            .swap_remove(
                sum
                    .iter()
                    .position(|x| *x == max )
                    .unwrap_or(0) // ewwww
            );
        ret.push(i_max);
    }
    // bruh. only need sum
    //ret.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(",")
    ret
        .iter()
        .sum::<i32>()
        .to_string()
}


#[cfg(test)]
mod tests {
    use super::{parser};

    #[test]
    fn test_input() {
        let input = r#"1000
        2000
        3000

        4000

        5000
        6000

        7000
        8000
        9000

        10000"#;
        assert_eq!(parser(input), vec![
            vec![2000,3000],
            vec![4000],
            vec![5000,6000],
            vec![7000,8000,9000]
        ]);
    }
}