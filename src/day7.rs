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

/*
1 1 1 
3 2 4
6 3 9
10 4 16
15 5 25
21 6 36
t_0 = 0
t_n = t_n-1 + n
an^2 + bn = a(n-1)^2 + b(n-1) + n
  = an^2 + (b-2a-1)n + a-b
a = b = 1/2
t_n = n(n+1)/2

E(x_i;y) = 1/2 (x_i-y)^2 + 1/2 |x_i-y|
dE/dy = sum (x_i-y) + 1/2 sgn(y-x_i)
   = mean(x) - y - (1/2n) [count x>y - count x<y]
*/

fn triangle_cost(positions: &[i32], center: i32) -> i32 {
    positions.iter()
        .map(|&i| (i-center).abs())
        .map(|i| i*(i+1) / 2)
        .sum()
} 

#[aoc(day7,part2)]
pub fn solve_part2(input: &str) -> i32 {
    let positions: Vec<i32> = input
        .split(",")
        .map(|i| i.parse().unwrap())
        .collect();
    let mean: i32 = positions.iter().sum::<i32>() / positions.len() as i32;
    let n_right: usize = positions.iter()
        .filter(|&&i| i > mean)
        .count();
    let n_left: usize = positions.iter()
        .filter(|&&i| i < mean)
        .count();

        match n_right.cmp(&n_left) {
            Ordering::Equal => mean,
            Ordering::Less => {
                let mut prev = i32::MAX;
                for i in mean..*positions.iter().max().unwrap() {
                    let cost = triangle_cost(&positions, i);
                    if cost > prev {return prev}
                    prev = cost;
                }
                unreachable!()
            },
            Ordering::Greater => {
                let mut prev = i32::MIN;
                for i in (mean..*positions.iter().min().unwrap()).rev() {
                    let cost = triangle_cost(&positions, i);
                    if cost > prev {return prev}
                    prev = cost;
                }
                unreachable!()
            }
        }
}


#[cfg(test)]
mod tests {
    use super::{solve_part1, solve_part2};

    #[test]
    fn test_part1() {
        let input = "16,1,2,0,4,2,7,1,2,14";
        assert_eq!(solve_part1(&input), 37);
    }

    #[test]
    fn test_part2() {
        let input = "16,1,2,0,4,2,7,1,2,14";
        assert_eq!(solve_part2(&input), 168);
    }
}