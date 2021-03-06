use std::collections::BTreeSet;

#[aoc(day8,part1)]
pub fn solve_part1(input: &str) -> u32 {

    let unique_digits: BTreeSet<usize> = BTreeSet::from([2,3,4,7]);

    input.lines()
        .map(|s| s.split_once(" | ").unwrap().1)
        .map(|s| {
            s.split_ascii_whitespace()
                .map(|digit| digit.len())
                .filter(|digit| unique_digits.contains(digit))
                .count() as u32
        })
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::solve_part1;

    const INPUT: &str = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(INPUT), 26);
    }

}