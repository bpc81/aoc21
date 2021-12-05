use std::iter::Iterator;
// use std::iter::IntoIterator;

#[derive(Debug,PartialEq,Eq)]
struct BingoBoard {
    rows: [[u8;5];5]
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

struct Game {
    numbers: Vec<u8>,
    boards: Vec<BingoBoard>
}

// #[aoc_generator(day4)]
// pub fn read_game(input: &str) -> Game {
//     let mut input = input.lines();
//     let numbers: Vec[u8] = input.next()
//         .split(",")
//         .map(|n| n.parse::<u8>())
//         .collect();
//     let _ = 
// }

#[cfg(test)]
mod tests {
    use super::{
        BingoBoard, BoardBuilder, Game,
        read_board
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

    #[test]
    fn test_read_board() {
        assert_eq!(read_board(BOARD1_INPUT), BOARD1);
    }
}