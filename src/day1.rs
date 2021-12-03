

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


#[cfg(test)]
mod tests{
    use super::part1;

    #[test]
    fn sample1() {
        let input = "13\n10\n21\n4\n99";
        assert_eq!(part1(input), 2);
    }
}