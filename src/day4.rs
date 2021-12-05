use std::iter::Iterator;
// use std::iter::IntoIterator;
use std::collections::HashSet;

#[derive(Debug,PartialEq,Eq)]
struct BingoBoard {
    rows: [[u8;5];5]
}

impl BingoBoard {
    fn _has_bingo(&self, numbers: &HashSet<u8>) -> bool {
        if self.rows.iter().any(
            |row| row.iter().all(
                |num| numbers.contains(&num)
            )
        ) {return true}

        for i in 0..5 {
            if self.rows.iter().all(
                |row| numbers.contains(&row[i])
            ) {return true}
        }
        false
    }

    fn _score(&self, numbers: &HashSet<u8>) -> u32 {
        self.rows.iter().flatten()
            .filter(|x| !numbers.contains(x))
            .map(|x| *x as u32)
            .sum()
    }

    pub fn score(&self, numbers: &HashSet<u8>) -> Option<u32> {
        match self._has_bingo(numbers) {
            true => Some(self._score(numbers)),
            false => None
        }
    }
}

struct BoardBuilder {
    rows: [[u8;5];5]
}

impl BoardBuilder {
    pub fn new() -> Self {
        Self {rows: [[0;5];5]}
    }

    pub fn create(self) -> BingoBoard {
        BingoBoard {rows: self.rows}
    }
}

fn read_board(input: [&str;5]) -> BingoBoard {
    let mut builder = BoardBuilder::new();

    for (row,line) in builder.rows.iter_mut().zip(input) {
        for (i,pos) in row.iter_mut().enumerate() {
            let val = &line[3*i..3*i+2];
            *pos = val.trim().parse::<u8>().unwrap();
        }
    }
    builder.create()
}

#[derive(Debug,PartialEq,Eq)]
pub struct Game {
    numbers: Vec<u8>,
    boards: Vec<BingoBoard>
}

#[aoc_generator(day4)]
pub fn read_game(input: &str) -> Game {
    let mut input = input.lines();
    let numbers: Vec<u8> = input.next().unwrap()
        .split(",")
        .map(|n| n.parse::<u8>().unwrap())
        .collect();
    let mut boards: Vec<BingoBoard> = Vec::new();
    while let Some(_) = input.next() {
        let rows = [
            input.next().unwrap(),
            input.next().unwrap(),
            input.next().unwrap(),
            input.next().unwrap(),
            input.next().unwrap(),
        ];
        boards.push(read_board(rows));
    }
    Game {numbers: numbers, boards: boards}
}

#[cfg(test)]
mod tests {
    use super::{
        BingoBoard, BoardBuilder, Game,
        read_board, read_game
    };

    const BOARD1_INPUT: [&str;5] = [
        "22 13 17 11  0",
        " 8  2 23  4 24",
        "21  9 14 16  7",
        " 6 10  3 18  5",
        " 1 12 20 15 19"];

    const BOARD1: BingoBoard = BingoBoard {
        rows: [
            [22,13,17,11,0],
            [8,2,23,4,24],
            [21,9,14,16,7],
            [6,10,3,18,5],
            [1,12,20,15,19]
        ]
    };

    const FULL_INPUT: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    #[test]
    fn test_read_board() {
        assert_eq!(read_board(BOARD1_INPUT), BOARD1);
    }


}