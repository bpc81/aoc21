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

#[derive(Copy, Clone)]
enum Criterion {
    Oxygen,
    CO2,
}

fn filter_numbers<'a>(input: &[&'a [u8]], bit: usize, criterion: Criterion) -> Vec<&'a [u8]> {
    let mut counter = BitCounter::new();
    input.iter().for_each(|num| match num[bit] {
        b'0' => counter.zeros += 1,
        b'1' => counter.ones += 1,
        _ => panic!("invalid input bit")
    });
    let keep_bit = match (criterion, counter.ones.cmp(&counter.zeros)) {
        (Criterion::Oxygen, Ordering::Greater) => b'1',
        (Criterion::Oxygen, Ordering::Equal) => b'1',
        (Criterion::Oxygen, Ordering::Less) => b'0',
        (Criterion::CO2, Ordering::Greater) => b'0',
        (Criterion::CO2, Ordering::Equal) => b'0',
        (Criterion::CO2, Ordering::Less) => b'1',
    };

    let mut result = Vec::<&[u8]>::new();

    input.iter()
        .filter(|num| num[bit] == keep_bit)
        .for_each(|num| result.push(&num));

    result
}

// input could also be mutable (clone before passing)
fn iterated_filter(input: &[&[u8]], criterion: Criterion) -> Vec<u8> {
    let n_bits: usize = input[0].len();
    let mut input: Vec<&[u8]> = input.to_vec();
    for bit in 0..n_bits {
        input = filter_numbers(&input, bit, criterion);
        match input.len() {
            0 => panic!("No numbers remaining"),
            1 => return input[0].to_vec(),
            _ => ()
        }
    }
    panic!("Exhausted all filters")
}

// #[aoc(day3, part2)]
// pub fn solve_part2(input: &str) -> u32 {
//     let input: Vec<&[u8]> = input.lines()
//         .map(|num| num.as_bytes())
//         .collect();
//     0
    
// }

#[cfg(test)]
mod tests {
    use super::{
        BitCounter, count_bits, solve_part1,
        filter_numbers, Criterion, iterated_filter
    };

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

    #[test]
    fn test_filter() {
        let input: Vec<&[u8]> = INPUT.lines().map(|num| num.as_bytes()).collect();
        let expected_result: Vec<&[u8]> = vec![
            input[0],
            input[5],
            input[6],
            input[10],
            input[11]
        ];
        assert_eq!(
            filter_numbers(&input, 0, Criterion::CO2),
            expected_result
        );
    }

    #[test]
    fn test_iterated_filter() {
        let input: Vec<&[u8]> = INPUT.lines().map(|num| num.as_bytes()).collect();
        let expected_result = "10111".as_bytes().to_vec();
        assert_eq!(iterated_filter(&input,Criterion::Oxygen), expected_result);
    }

}