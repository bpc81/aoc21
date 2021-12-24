

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

#[cfg(test)]
mod tests {
    use super::{timestep, solve_part1};

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



}