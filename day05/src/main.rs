use core::num;
use std::collections::btree_map::Range;
use std::ops::RangeInclusive;
use nom::multi::{many0, separated_list1};
use nom::{IResult};
use nom::character::complete::{digit1, line_ending, u32, char};
use nom::bytes::complete::tag;
use nom::sequence::tuple;
use nom::combinator::{map};
use nom::Parser;
use nom::sequence::separated_pair;
use std::fmt::{self, write};

use itertools::{Itertools, MinMaxResult};
use itertools::MinMaxResult::{NoElements, OneElement, MinMax};
use std::iter::once;





#[derive(Debug)]
struct ParsedInput {
    hydrothermals: Vec<Line>
}

#[derive(Debug)]
struct Line {
    x: RangeInclusive<u32>,
    y: RangeInclusive<u32>,
}

#[derive(Debug)]
struct Coord {
    x: u32,
    y: u32,
}


fn parse_input(input: &str) -> IResult<&str, ParsedInput> {

    let start_point = u32.and(char(',')).and(u32).map(|((x,_), y)| Coord{ x, y});
    let sep = tag(" -> ");
    let end_point = u32.and(char(',')).and(u32).map(|((x,_), y)| Coord{ x, y});
    let row = separated_pair(start_point, sep, end_point).map(|(x, y)| {
                        let mut x_start = x.x;
                        let mut x_end = y.x;
                        
                        let mut y_start = x.y;
                        let mut y_end = y.y;

                        if (x.x == y.x) | (x.y == y.y) {
                            if x.x > y.x {
                                x_start = y.x;
                                x_end = x.x;
                            }
    
                            
                            if x.y > y.y {
                                y_start = y.y;
                                y_end = x.y;
                            }
                        } 
                        else {
                            x_start = x.x;
                            x_end = y.x;

                            y_start = x.y;
                            y_end = y.y;
                        }
                        
                        
                        Line { 
                            x: RangeInclusive::new(x_start, x_end),
                            y: RangeInclusive::new(y_start, y_end)} 
    });
    let mut parser = separated_list1(line_ending, row)
                                                        .map(|v| {
                                                            ParsedInput {
                                                                hydrothermals: v,
                                                            }
                                                        } );
    parser.parse(input)

}

#[derive(Clone, Copy, PartialEq)]
enum Tile {
    Open,
    Cover(u32),
}

impl Default for Tile {
    fn default() -> Self {
        Self::Open
    }
}

impl fmt::Debug for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            Tile::Open => ".".to_string(),
            Tile::Cover(w) => w.to_string(),
        };
        write!(f, "{}", c)
    }
}

struct Map {
    size_x: RangeInclusive<u32>,
    size_y: RangeInclusive<u32>,
    tiles: Vec<Tile>,
}

impl Map {
    fn new(bound_x: MinMaxResult<u32>, bound_y: MinMaxResult<u32>, tiles: &Vec<Line>) -> Self {

        let size_x = match bound_x.into_option() {
            Some((x, y)) => RangeInclusive::new(x, y),
            None => panic!("Error"),
        };

        let size_y = match bound_y.into_option() {
            Some((x, y)) => RangeInclusive::new(x, y),
            None => panic!("Error"),
        };

        // let num_tiles = ((size_x.end() - size_x.start()) + 1)*((size_y.end() - size_y.start()) + 1);
        
        // assert_eq!(size_x.start(), &0);
        let n_rows = size_y.end().clone()+1;
        // assert_eq!(size_y.start(), &0);
        let n_cols = size_x.end().clone()+1;
        let num_tiles = n_rows*n_cols;
        Self {
            size_x,
            size_y,
            tiles: (0..num_tiles).into_iter()
                        .map(|_| Default::default())
                        .collect(),
        }

    }

    fn get(&self, pos: Coord) -> Option<&Tile> {
        let idx =  (pos.x + pos.y * (self.size_x.end() + 1)) as usize;
        self.tiles.get(idx)
    }

    fn set_all(&mut self, tiles: &Vec<Line>) {


        for el in tiles {
            if (el.x.start() == el.x.end()) | (el.y.start() == el.y.end()) {
                for x in el.x.start().clone()..=el.x.end().clone() {
                    for y in el.y.start().clone()..=el.y.end().clone() {
                        // if x == 7 {
                        //     // dbg!(el);
                        //     // dbg!(x, y);
                        // }
                        
                        self.set((x,y).into())
                    }
                }
            } else {
                // dbg!(el);
                let mut diff;
                
                if el.x.end() < el.x.start() {
                    diff = el.x.start() - el.x.end();
                } else {
                    diff = el.x.end() - el.x.start();
                }
                // dbg!(el);
                // dbg!(diff);

                let mut vec_x: Vec<u32> = Vec::new();

                if el.x.start() <= el.x.end() {
                    for i in 0..=diff {
                        vec_x.push(el.x.start().clone() + i);
                    }
                } else {
                    for i in 0..=diff {
                        vec_x.push(el.x.start().clone() - i);
                    }
                }

                let mut vec_y: Vec<u32> = Vec::new();

                if el.y.start() <= el.y.end() {
                    for i in 0..=diff {
                        vec_y.push(el.y.start().clone() + i);
                    }
                } else {
                    for i in 0..=diff {
                        vec_y.push(el.y.start().clone() - i);
                    }
                }

                for i in 0..vec_x.len() {
                    // dbg!((vec_x[i], vec_y[i]));
                    self.set((vec_x[i], vec_y[i]).into());
                }
            } 
        }

    }

    fn set(&mut self, pos: Coord) {
        let idx = (pos.x + pos.y * (self.size_x.end()+1)) as usize;
        let t = self.tiles.remove(idx);
        let new_t = match t {
                Tile::Open => Tile::Cover(1),
                Tile::Cover(n) => Tile::Cover(n+1),
            };
        self.tiles.insert(idx, new_t);
        
    }
}

impl From<(u32, u32)> for Coord {
    fn from((x, y): (u32, u32)) -> Self {
        Self {x, y}
    }
}

impl fmt::Debug for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..=*self.size_x.end(){
            for col in 0..=*self.size_y.end() {
                write!(f, "{:?}", self.get((col, row).into()).unwrap())?;
            }
            writeln!(f)?;
        }
     Ok(())
    }
}

fn main() {
    let input = include_str!("input.txt").trim();
    let (_, vec_lines) = parse_input(input).unwrap();
    // println!("{:?}", &pi);

    let t = RangeInclusive::new(7, 9);
    
    // dbg!(t.contains(&3));
    let bounds_x = vec_lines.hydrothermals.iter()
                                    .map(|el| (el.x.start().clone(), el.x.end().clone()))
                                    .flat_map(|tup| once(tup.0).chain(once(tup.1)))
                                    .minmax();

    let bounds_y = vec_lines.hydrothermals.iter()
                                    .map(|el| (el.y.start().clone(), el.y.end().clone()))
                                    .flat_map(|tup| once(tup.0).chain(once(tup.1)))
                                    .minmax();
    // dbg!(bounds_x);
    // dbg!(bounds_y);

    // let hydros = vec_lines.hydrothermals.into_iter().filter(|l| (l.x.start() == l.x.end()) | (l.y.start() == l.y.end())).collect::<Vec<Line>>();
    let hydros = vec_lines.hydrothermals;
    println!("{:?}", &hydros);

    let mut m = Map::new(bounds_x, bounds_y, &hydros);
    m.set_all(&hydros);
    // println!("{:?}", &m);

    let c = m.tiles.into_iter().filter_map(|p| match p {
        Tile::Open => None,
        Tile::Cover(n) => { if n >= 2 { Some(Tile::Cover(n))} else {None}},
    } ).count();

    println!("{:?}", &c);

}



