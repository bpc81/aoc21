use std::collections::VecDeque;

fn timestep(fish: &mut Vec<u8>) {
    let mut spawn: usize = 0;
    for f in fish.iter_mut() {
        if *f > 0 { 
            *f -= 1
        }
        else {
            *f = 6;
            spawn += 1;
        }
    }
    fish.resize(fish.len()+spawn, 8);
}



#[aoc(day6,part1)]
pub fn solve_part1(input:&str) -> usize {
    let mut fish: Vec<u8> = input.split(",").map(|s| s.parse().unwrap()).collect();
    for _ in 0..80 {
        timestep(&mut fish);
    }
    fish.len()
}

fn update(population: &mut VecDeque<usize>) {
    // let spawn = population.pop_front().unwrap();
    // population.push_back(spawn);
    // population[6] += spawn;
    let spawn = population[0];
    population.rotate_left(1);
    population[6] += spawn;
}

#[aoc(day6,part2)]
pub fn solve_part2(input: &str) -> usize {
    // let mut population = [0usize;9];
    let mut population = VecDeque::from([0usize;9]);
    for s in input.split(",") {
        population[s.parse::<usize>().unwrap()] += 1;
    }
    for _ in 0..256 {
        update(&mut population);
    }
    population.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::{timestep, solve_part1, update, solve_part2};
    use std::collections::VecDeque;

    #[test]
    fn test_timestep() {
        let mut fish: Vec<u8> = vec![3u8,4,3,1,2];
        timestep(&mut fish);
        assert_eq!(fish, vec![2,3,2,0,1]);
        timestep(&mut fish);
        assert_eq!(fish, vec![1,2,1,6,0,8]);
    }

    #[test]
    fn test_part1() {
        let input = "3,4,3,1,2";
        assert_eq!(solve_part1(&input), 5934);
    }

    #[test]
    fn test_update() {
        let mut population: VecDeque<usize> = VecDeque::from([0,1,1,2,1]);
        population.resize(9,0);
        update(&mut population);
        assert_eq!(population, VecDeque::from([1,1,2,1,0,0,0,0,0]));
        update(&mut population);
        assert_eq!(population, VecDeque::from([1,2,1,0,0,0,1,0,1]));
    }

    #[test]
    fn test_part2() {
        let input = "3,4,3,1,2";
        assert_eq!(solve_part2(&input), 26984457539);
    }
}