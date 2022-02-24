
use std::ops::Sub;

// use peg::*;

#[derive(Clone, Copy, PartialEq, Debug)]
enum SubmarineCmds {
    forward(i64),
    down(i64),
    up(i64),
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Vec2 {
    horizontal_x: i64,
    height_y: i64,
    aim: i64,
}


// impl Default for Vec2 {
//     fn default() -> Self {
//         Vec2 { horizontal_x: 0, height_y: 0}
//     }
// }

impl Vec2 {

    fn new() -> Self {
        Self { horizontal_x: 0, height_y: 0, aim: 0}
    }

    fn set(&mut self, cmd: SubmarineCmds) -> SubmarineCmds {
        match cmd {
            SubmarineCmds::forward(units) => { self.horizontal_x += units;
                                                    self.height_y += self.aim*units },
            SubmarineCmds::down(units) => { self.aim += units },
            SubmarineCmds::up(units) => { self.aim -= units},
        }

        cmd
    }

    fn result(&self) -> i64 {
        self.height_y * self.horizontal_x
    }
    
}

fn parse_line(s: &str) -> anyhow::Result<SubmarineCmds> {
    peg::parser!{
        grammar parser() for str {

            rule num() -> i64
            = s:$(['0'..='9']+) { s.parse().unwrap() }

            rule cmd() -> SubmarineCmds
            = "forward " num:num() { SubmarineCmds::forward(num) }
            / "down " num:num() { SubmarineCmds::down(num) }
            / "up " num:num() { SubmarineCmds::up(num) }

            pub(crate) rule line() -> SubmarineCmds
            = l:cmd() { l } 
        }
    }

    Ok(parser::line(s)?)
    
}

fn main() {
    let mut v = Vec2::new();
    // let s: Vec<SubmarineCmds> = include_str!("input_test.txt").lines()
    //                                     .map(parse_line)
    //                                     .map(|x| x.unwrap())
    //                                     .collect::<Vec<SubmarineCmds>>();

    let r = include_str!("input.txt").lines()
                                        .map(parse_line)
                                        .map(|c|c.unwrap())
                                        .map(|c| v.set(c))
                                        .collect::<Vec<SubmarineCmds>>();

    // println!("{:?}", &r);
    println!("{:?}", &v);
    println!("{}", &v.result());
}
