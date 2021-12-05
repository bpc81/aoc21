use std::cmp::Ordering;

fn binary_string_to_int(bits: &str) -> u64 {
    bits.bytes()
        .rev()
        .enumerate()
        .filter( |(_,bit)| *bit==b'1')
        .map( |(i,_)| 1u64 << i)
        .sum()
}

#[aoc_generator(day3)]
fn parse(input: &str) -> Vec<u64> {
    input.lines()
        .map(binary_string_to_int)
        .collect()
}

fn most_frequent_value(numbers: &[u64], position: u8) -> u64 {
    let n = numbers.len();
    let ones = numbers.iter()
        .filter(|x| *x & (1u64 << position) > 0)
        .count();
    match ones.cmp(&(n-ones)) {
        Ordering::Greater => 1,
        Ordering::Equal => 1,
        Ordering::Less => 0
    }
}

fn get_num_bits(n: u64) -> u8 {
    for i in 0..64u8 {
        if n >> i == 0 {return i}
    }
    unreachable!()
}

fn find_single_number(mut numbers: Vec<u64>, invert: bool) -> u64 {
    let n_bits: u8 = get_num_bits(*numbers.iter().max().unwrap());
    let mut val: u64;
    for i in (0..n_bits).rev() {
        val = most_frequent_value(&numbers, i);
        if invert {val = 1 - val};
        numbers = numbers.into_iter().filter(|x| (x >> i) % 2 == val).collect();
        match numbers.len() {
            1 => return numbers[0],
            0 => panic!("All numbers eliminated"),
            _ => ()
        }
    }
    panic!("Exhausted all bits")
}

#[aoc(day3,part1)]
pub fn solve_part1(input: &Vec<u64>) -> u64 {
    let n_bits: u8 = get_num_bits(*input.iter().max().unwrap());
    let mut gamma = 0u64; 
    let mut epsilon = 0u64;
    let mut val: u64;
    for i in 0..n_bits {
        val = most_frequent_value(input, i);
        gamma += val << i;
        epsilon += (1-val) << i;
    }
    gamma * epsilon
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &Vec<u64>) -> u64 {
    let gamma = find_single_number(input.clone(), false);
    let epsilon = find_single_number(input.clone(), true);
    gamma * epsilon
}

#[cfg(test)]
mod tests {
    use super::{
        parse,
        most_frequent_value,
        find_single_number,
        solve_part1,
        solve_part2
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
    
    const NUMBERS: [u64;12] = [
        4,30,22,23,21,15,7,28,16,25,2,10
    ];

    #[test]
    fn test_parse() {
        assert_eq!(parse(&INPUT), NUMBERS);
    }

    #[test]
    fn test_frequent_values() {
        assert_eq!(most_frequent_value(&NUMBERS,4),1);
        assert_eq!(most_frequent_value(&NUMBERS,3),0);
        assert_eq!(most_frequent_value(&NUMBERS,2),1);
        assert_eq!(most_frequent_value(&NUMBERS,1),1);
        assert_eq!(most_frequent_value(&NUMBERS,0),0);
    }

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(&NUMBERS.to_vec()), 198u64);
    }

    #[test]
    fn test_reduce() {
        assert_eq!(find_single_number(NUMBERS.to_vec(),false), 23u64);
        assert_eq!(find_single_number(NUMBERS.to_vec(),true), 10u64);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(&NUMBERS.to_vec()), 230);
    }
}