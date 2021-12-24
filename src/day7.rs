use std::cmp::{Reverse, Ordering};
use std::collections::BinaryHeap;

#[aoc(day7,part1)]
pub fn solve_part1(input: &str) -> i32 {
    let mut left: BinaryHeap<i32> = BinaryHeap::new();
    let mut right: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
    
    for pos in input.split(",").map(|n| n.parse::<i32>().unwrap()) {
        match pos.cmp(left.peek().unwrap_or(&i32::max_value())) {
            Ordering::Less => {left.push(pos)},
            Ordering::Greater => {right.push(Reverse(pos))},
            Ordering::Equal => {
                match left.len().cmp(&right.len()) {
                    Ordering::Less => {left.push(pos)},
                    _ => {right.push(Reverse(pos))}
                }
            }
        }
        if right.len() > left.len() + 1 {
            let Reverse(pos) = right.pop().unwrap();
            left.push(pos);
        }
        if left.len() > right.len() + 1 {
            right.push(Reverse(left.pop().unwrap()));
        }
    }
    let median = left.pop().unwrap();
    
    let mut fuel: i32 = 0;
    for pos in left.into_iter() { fuel += median - pos }
    for Reverse(pos) in right.into_iter() { fuel += pos - median }

    fuel
}

#[cfg(test)]
mod tests {
    use super::{solve_part1};

    #[test]
    fn test_part1() {
        let input = "16,1,2,0,4,2,7,1,2,14";
        assert_eq!(solve_part1(&input), 37);
    }
}