use std::cmp::Ordering;

#[derive(PartialEq, Eq, Debug)]
pub struct BitCounter {
    zeros: u32,
    ones: u32
}

impl BitCounter {
    fn new() -> Self {
        Self {zeros:0, ones:0}
    }
}

#[aoc_generator(day3)]
pub fn count_bits(input: &str) -> Vec<BitCounter> {
    let mut lines = input.lines().peekable();
    let n_bits = lines.peek().unwrap().len();
    let mut counters = Vec::<BitCounter>::new();
    for _ in 0..n_bits {counters.push(BitCounter::new())}

    for line in lines {
        for (counter, bit) in counters.iter_mut().zip(line.bytes()) {
            match bit {
                b'0' => counter.zeros += 1,
                b'1' => counter.ones += 1,
                _ => panic!("invalid input character")
            }
        } 
    }
    counters
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &[BitCounter]) -> u32 {
    let mut gamma: u32 = 0;
    let mut epsilon: u32 = 0;
    for (pos,counts) in input.iter().rev().enumerate() {
        match counts.ones.cmp(&counts.zeros) {
            Ordering::Greater => {
                gamma += 1u32 << pos;
            },
            Ordering::Less => {
                epsilon += 1u32 << pos;
            },
            Ordering::Equal => panic!("Indeterminate comparison")
        }
    }
    gamma * epsilon
}

#[cfg(test)]
mod tests {
    use super::{BitCounter, count_bits, solve_part1};

    const INPUT: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";
    const COUNTS: [BitCounter; 5] = [
        BitCounter {zeros:5, ones:7},
        BitCounter {zeros:7, ones:5},
        BitCounter {zeros:4, ones:8},
        BitCounter {zeros:5, ones:7},
        BitCounter {zeros:7, ones:5}
    ];

    #[test]
    fn test_input() {
        assert_eq!(count_bits(INPUT), COUNTS);
    }

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(&COUNTS), 198u32);
    }
}