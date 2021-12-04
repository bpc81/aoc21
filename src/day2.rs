pub struct Pos {
    horiz: i32,
    depth: i32
}

pub struct PosAim {
    horiz: i32,
    depth: i32,
    aim: i32
}

#[derive(PartialEq, Eq, Debug)]
pub enum Step {
    Horiz(i32),
    Depth(i32)
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Step> {
    input.lines().map(|line| {
        let (direction, distance) = line.split_once(' ').unwrap();
        match (direction, distance.parse::<i32>().unwrap()) {
            ("forward", x) => Step::Horiz(x),
            ("up", x) => Step::Depth(-x),
            ("down",x) => Step::Depth(x),
            _ => panic!("invalid instruction")
        }
    }
    ).collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[Step]) -> i32 {
    let mut pos = Pos {horiz: 0, depth: 0};
    for step in input {
        match step {
            Step::Horiz(x) => pos.horiz += x,
            Step::Depth(x) => pos.depth += x
        }
    }
    pos.horiz * pos.depth
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[Step]) -> i32 {
    let mut pos = PosAim {horiz:0, depth:0, aim:0};
    for step in input {
        match step {
            Step::Horiz(x) => {
                pos.horiz += x;
                pos.depth += pos.aim * x;
            },
            Step::Depth(x) => pos.aim += x
        }
    }
    pos.horiz * pos.depth
}

#[cfg(test)]
mod tests{
    use super::{solve_part1, solve_part2, input_generator, Step};

    const INPUT: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

    const STEPS: [Step;6] = [
        Step::Horiz(5),
        Step::Depth(5),
        Step::Horiz(8),
        Step::Depth(-3),
        Step::Depth(8),
        Step::Horiz(2)
    ];


    #[test]
    fn test_input() {
        assert_eq!(input_generator(INPUT), STEPS);
    }

    #[test]
    fn test_part1_agg() {
        assert_eq!(solve_part1(&STEPS), 150);
    }

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(&input_generator(INPUT)), 150);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(&input_generator(INPUT)), 900);
    }

}