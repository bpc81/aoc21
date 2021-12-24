use std::collections::HashMap;
use std::cmp;

#[derive(Debug,PartialEq,Eq)]
enum Line {
    Horiz {y: i32, x1: i32, x2: i32},
    Vert {x: i32, y1: i32, y2: i32},
    DiagNE {x1: i32, y1: i32, len: i32},
    DiagSE {x1: i32, y1: i32, len: i32},
    Skew //{x1:i32, y1:i32, x2:i32, y2:i32}
}

impl Line {
    pub fn parse(line: &str) -> Line {
        let (p1,p2) = line.split_once(" -> ").unwrap();
        let (x1,y1) = p1.split_once(",").unwrap();
        let (x2,y2) = p2.split_once(",").unwrap();
        let x1: i32 = x1.parse().unwrap();
        let y1: i32 = y1.parse().unwrap();
        let x2: i32 = x2.parse().unwrap();
        let y2: i32 = y2.parse().unwrap();

        if y1 == y2 {
            return Line::Horiz {
                y:y1, x1:cmp::min(x1,x2), x2:cmp::max(x1,x2)
            }
        };
        if x1 == x2 {
            return Line::Vert {
                x:x1, y1:cmp::min(y1,y2), y2:cmp::max(y1,y2)
            }
        };
        if x2-x1 == y2-y1 {
            return Line::DiagNE {
                x1:cmp::min(x1,x2), y1:cmp::min(y1,y2), len: (x2-x1).abs()
            }
        };
        if x2-x1 == y1-y2 {
            return Line::DiagSE {
                x1:cmp::min(x1,x2), y1:cmp::max(y1,y2), len: (x2-x1).abs()
            }
        };

        Line::Skew //{x1, y1, x2, y2}
    }
}

#[aoc(day5,part1)]
pub fn solve_part1(input: &str) -> usize {
    let mut points: HashMap<(i32,i32),i32> = HashMap::new();
    for line in input.lines() { match Line::parse(line) {
        Line::Horiz {y,x1,x2} => {
            for x in x1..=x2 {
                let count = points.entry((x,y)).or_insert(0);
                *count += 1;
            }
        }
        Line::Vert {x,y1,y2} => {
            for y in y1..=y2 {
                let count = points.entry((x,y)).or_insert(0);
                *count += 1;                
            }
        }
        _ => ()
        // Line::DiagNE {x1,y1,len} => {
        //     for d in 0..=len {
        //         let count = points.entry((x1+d,y1+d)).or_insert(0);
        //         *count += 1;
        //     }
        // }
        // Line::DiagSE {x1,y1,len} => {
        //     for d in 0..=len {
        //         let count = points.entry((x1+d,y1-d)).or_insert(0);
        //         *count += 1;
        //     }
        // }
    }}
    points.into_iter()
        .filter(|(_,count)| *count > 1)
        .count()
}

#[aoc(day5,part2)]
pub fn solve_part2(input: &str) -> usize {
    let mut points: HashMap<(i32,i32),i32> = HashMap::new();
    for line in input.lines() { match Line::parse(line) {
        Line::Horiz {y,x1,x2} => {
            for x in x1..=x2 {
                let count = points.entry((x,y)).or_insert(0);
                *count += 1;
            }
        }
        Line::Vert {x,y1,y2} => {
            for y in y1..=y2 {
                let count = points.entry((x,y)).or_insert(0);
                *count += 1;                
            }
        }
        Line::DiagNE {x1,y1,len} => {
            for d in 0..=len {
                let count = points.entry((x1+d,y1+d)).or_insert(0);
                *count += 1;
            }
        }
        Line::DiagSE {x1,y1,len} => {
            for d in 0..=len {
                let count = points.entry((x1+d,y1-d)).or_insert(0);
                *count += 1;
            }
        }
        _ => ()
    }}
    points.into_iter()
        .filter(|(_,count)| *count > 1)
        .count()
}

#[cfg(test)]
mod tests {
    use super::{Line, solve_part1, solve_part2};

    const INPUT: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn test_parse() {
        let line1 = "0,9 -> 5,9";
        assert_eq!(Line::parse(line1), Line::Horiz {y:9,x1:0,x2:5});
        let line2 = "0,8 -> 8,0";
        assert_eq!(Line::parse(line2), Line::DiagSE {x1:0,y1:8,len:8});
        let line3 = "6,4 -> 2,0";
        assert_eq!(Line::parse(line3), Line::DiagNE {x1:2,y1:0,len:4});
    }

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(INPUT), 5);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(INPUT), 12);
    }
}

