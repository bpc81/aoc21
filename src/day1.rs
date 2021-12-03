use std::collections::VecDeque;

#[aoc(day1,part1)]
pub fn part1(input: &str) -> i32 {
    let mut increases: i32 = 0;
    let mut data = input.lines()
        .map(|s| s.parse::<i32>().unwrap());
    let mut prev: i32 = data.next().unwrap();

    data.for_each(|x| {
        if x > prev {increases += 1;}
        prev = x;
    });
    increases
}



#[aoc(day1,part2)]
pub fn part2(input: &str) -> i32 {
    
    let mut data = input.lines()
        .map(|s| s.parse::<i32>().unwrap());
    let mut buf: VecDeque<i32> = VecDeque::with_capacity(3);

    for _ in 0..2 {
        match data.next() {
            Some(x) => buf.push_back(x),
            None => return 0
        }
    }

    let mut increases: i32 = 0;
    let mut prev_sum = i32::MAX;
    for x in data {
        buf.push_back(x);
        let current_sum = buf.iter().sum();
        if current_sum > prev_sum {increases += 1};
        prev_sum = current_sum;
        let _ = buf.pop_front();
    }
    increases
}

#[cfg(test)]
mod tests{
    use super::{part1,part2};

    #[test]
    fn sample1() {
        let input = "13\n10\n21\n4\n99";
        assert_eq!(part1(input), 2);
    }

    #[test]
    fn sample2() {
        let input = "1\n0\n0\n0\n2\n0";
        assert_eq!(part2(input),1);
    }
}