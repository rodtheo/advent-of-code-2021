
use std::collections::HashMap;
use bitvec::prelude::*;
use std::char;

use nom::{IResult, multi::many1, bytes::streaming::{tag, take},  character::complete::anychar};

fn parse_input(input: &str) -> IResult<&str, Vec<char>> {
    many1(anychar)(input)
}

// fn to_u32(slice: &[u8]) -> u32 {
//     slice.iter().rev().fold(0, |acc, &b| acc*2 + b as u32)
// }

fn diagnostic_o2(rr: &Vec<Vec<char>>) -> BitVec::<Msb0, u8> {
    let mut r = rr.clone();
    let mut vec_hash= Vec::<HashMap<char, usize>>::new();
    for bit_idx in 0..r[0].len() {
        let mut hm: HashMap<char, usize> = HashMap::new();
        vec_hash.push(hm);
        for bv_idx in 0..r.len() {
            let cnt = vec_hash.get_mut(bit_idx).unwrap()
                                        .entry(r[bv_idx][bit_idx]).or_insert(0);
            *cnt += 1;
            // println!("{:?}", &bv_idx)
        }
        let el = &vec_hash[bit_idx];
        let bit_0 = el.get(&'0').unwrap_or(&0);
        let bit_1 = el.get(&'1').unwrap_or(&0);
        let mut wbv: char = '1';
        if bit_0 <= bit_1 {
            wbv = '1';
        } else {
            wbv = '0';
        }
        println!("before filtering = {:?}", &r);
        if r.len() == 1 {
            break
        } else {
            r = r.iter().filter(|&bv| {
                bv.get(bit_idx).unwrap() == &wbv
            }).cloned().collect::<Vec<Vec<char>>>();
        }
        
    }
    
    r[0].clone().iter().map(|x| (*x).to_digit(10)
                .unwrap()).map(|x| {if x == 1 {
                    true } else { false }
                }).collect::<BitVec::<Msb0, u8>>()
}

fn diagnostic_co2(rr: &Vec<Vec<char>>) -> BitVec::<Msb0, u8> {
    let mut r = rr.clone();
    let mut vec_hash= Vec::<HashMap<char, usize>>::new();
    for bit_idx in 0..r[0].len() {
        let mut hm: HashMap<char, usize> = HashMap::new();
        vec_hash.push(hm);
        for bv_idx in 0..r.len() {
            let cnt = vec_hash.get_mut(bit_idx).unwrap()
                                        .entry(r[bv_idx][bit_idx]).or_insert(0);
            *cnt += 1;
            // println!("{:?}", &bv_idx)
        }
        let el = &vec_hash[bit_idx];
        let bit_0 = el.get(&'0').unwrap_or(&0);
        let bit_1 = el.get(&'1').unwrap_or(&0);
        let mut wbv: char = '0';
        if bit_0 <= bit_1 {
            wbv = '0';
        } else {
            wbv = '1';
        }
        println!("before filtering = {:?}", &r);
        if r.len() == 1 {
            break
        } else {
            r = r.iter().filter(|&bv| {
                bv.get(bit_idx).unwrap() == &wbv
            }).cloned().collect::<Vec<Vec<char>>>();
        }
        
    }

    r[0].clone().iter().map(|x| (*x).to_digit(10)
                .unwrap()).map(|x| {if x == 1 {
                    true } else { false }
                }).collect::<BitVec::<Msb0, u8>>()
}

fn main() {
    let r = include_str!("input.txt").lines().map(parse_input)
                                                .map(|x| x.unwrap().1)
                                                // .map(|x| x as u8)
                                                .collect::<Vec<Vec<char>>>();

    // let co2 = r.clone();


    let o2 = diagnostic_o2(&r);
    let co2 = diagnostic_co2(&r);
    println!("O2 diagnostic = {:?}", &o2);
    println!("CO2 diagnostic = {:?}", &co2);

    // let o2_bv = BitVec::<Msb0, u8>::from_vec(o2);
    // let co2_bv = BitVec::<Msb0, u8>::from_vec(co2);

    // println!("O2 = {:?}", &o2_bv);
    // println!("CO2 = {:?}", &co2_bv);

    let value_res = o2.into_iter().rev().fold(0, |acc, b| acc*2 + b as u32);
    let value_complement = co2.into_iter().rev().fold(0, |acc, b| acc*2 + b as u32);

    // let mut res_bv: BitVec<Msb0, u8> = BitVec::new();
    // for el in vec_hash {
    //     let bit_0 = el.get(&'0').unwrap_or(&0);
    //     let bit_1 = el.get(&'1').unwrap_or(&0);

    //     if bit_0 > bit_1 {
    //         res_bv.push(false)
    //     } else {
    //         res_bv.push(true)
    //     }
    // }

    // let res_bv_complement = !res_bv.clone();
   
    // println!("{:?}", &res_bv);
    // println!("{:?}", &res_bv_complement);

    // // let value_res = res_bv.load::<usize>();
    // // let value_complement = res_bv_complement.load::<usize>();
    // let value_res = res_bv.into_iter().rev().fold(0, |acc, b| acc*2 + b as u32);
    // let value_complement = res_bv_complement.into_iter().rev().fold(0, |acc, b| acc*2 + b as u32);
    // // res_bv.iter()
    // // slice.iter().rev().fold(0, |acc, &b| acc*2 + b as u32)

    println!("{} and {}", &value_res, &value_complement);

    let result = 4023 * 690;
    println!("Result = {}", &result);
    // 4023 * 690
}
