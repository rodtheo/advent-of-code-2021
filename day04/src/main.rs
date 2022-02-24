use nom::{IResult};
use nom::character::complete::{char, line_ending, space1, space0, u32};
use nom::multi::{separated_list1};
use ndarray::{Array1, Array2, Axis};
use nom::Parser;
use nom::combinator::{map, map_res};
use anyhow::Result;

const BOARD_SIZE: usize = 5;

#[derive(Debug)]
struct ParsedInput {
    sequence: Vec<u32>,
    boards: Vec<Board>,
}

#[derive(Debug)]
struct Board {
    board: Array2<u32>,
    marked: Array2<u32>,
}

#[derive(Debug)]
struct Pos2 {
    i: usize,
    j: usize,
}

impl From<(usize, usize)> for Pos2 {
    fn from((x, y): (usize, usize)) -> Self {
        Self {i: x, j: y}
    }
}


impl Board {
    fn mark(&mut self, pos: &Pos2) {
        self.marked[[pos.i, pos.j]] = 0;
    }

    fn has_point(&self, sampled: &u32) -> Option<Pos2> {
        for (pos, n) in self.board.indexed_iter() {
            let p: Pos2 = pos.into();
            if n == sampled {
                return Some(p)
            }
        }
        None
    }

    fn is_bingo(&self) -> bool {
        let sum_axis_x = self.marked.sum_axis(Axis(0))
                                    .into_iter().filter(|&x| x == 0).count();
        if sum_axis_x > 0 {
            // dbg!(self);
            return true
        }
        let sum_axis_y = self.marked.sum_axis(Axis(1))
                                    .into_iter().filter(|&x| x == 0).count();
        if sum_axis_y > 0 {
            // dbg!(self);
            return true
        }
        false
    }
}

fn parse_input(input: &str) -> IResult<&str, ParsedInput> {
    // dbg!(input);
    let seq = separated_list1(char(','), u32);
    let gap = line_ending.and(line_ending);
    let row = space0.and(separated_list1(space1, u32));
    let rows = separated_list1(line_ending, row);
    // let board = separated_list1(gap_rows, )
    let board = map(rows, |x| { 
        let v = x.into_iter().flat_map(|(_, r)| r).collect::<Vec<u32>>();
        Board {
            board: Array2::from_shape_vec((BOARD_SIZE, BOARD_SIZE), v).unwrap(),
            marked: Array2::ones((BOARD_SIZE, BOARD_SIZE)),
        }
     });
    let boards = separated_list1(line_ending.and(line_ending), board);
    let mut parser = map(seq.and(gap).and(boards), |((seq, (_, _)), boards)| {
        // dbg!(x);
        ParsedInput {
            sequence: seq, 
            boards: boards,
        }}
    );
    parser.parse(input)
}

fn sorteio_part1(sequence: &Vec<u32>, boards: &mut Vec<Board>) -> anyhow::Result<u32> {
    for sampled in sequence.iter() {
        println!("Sampling number = {}", sampled);
        for board in boards.iter_mut() {
            if let Some(pos) = board.has_point(sampled) {
                board.mark(&pos);
                let bingo = board.is_bingo();
                if bingo {
                    let rr = &board.board*&board.marked;
                    return Ok(sampled*rr.sum() as u32)
                }
                // dbg!(board);
            }
        }
    }

    Err(anyhow::anyhow!("No lucky guy"))
}

fn sorteio_part2(sequence: &Vec<u32>, boards: &mut Vec<Board>) -> anyhow::Result<u32> {
    let mut res: u32 = 0;
    for sampled in sequence.iter() {
        println!("Sampling number = {}", sampled);
        for board in boards.iter_mut() {
            if let Some(pos) = board.has_point(sampled) {
                if !board.is_bingo() {

                    board.mark(&pos);
                    let bingo = board.is_bingo();
                    if bingo {
                        let rr = &board.board*&board.marked;
                        dbg!(board);
                        res = sampled*rr.sum() as u32;
                    }
                }
                
                // dbg!(board);
            }
        }
    }
    if res == 0 {
        Err(anyhow::anyhow!("No lucky guy"))
    } else {
        Ok(res)
    }
    
}


fn main() {
    let input = include_str!("input.txt").trim();
    let (_, res) = parse_input(&input).unwrap();

    let seqs = res.sequence;
    let mut boards = res.boards;
    // let board_test = dbg!(&res.boards[0]);
    let mut bingo = false;
    
    

    if let Ok(b) = sorteio_part2(&seqs, &mut boards) {
        println!("{}", b);
    }
   
    
    // dbg!(boards);

}
