// elu: a clone of muri, assembler for rung/the Nga VM
// instruction packing not implemented

extern crate byteorder;

use byteorder::{LittleEndian, WriteBytesExt};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;

pub fn save(target: &[i32; IMAGE_SIZE], path: &str) {
    let mut file = File::create(path).unwrap();

    for i in target.iter() {
        file.write_i32::<LittleEndian>(*i);
    }
}

fn opcode_for(s: &str) -> i32 {
    match s {
        ".." =>  0, "li" =>  1, "du" =>  2, "dr" =>  3,
        "sw" =>  4, "pu" =>  5, "po" =>  6, "ju" =>  7,
        "ca" =>  8, "cc" =>  9, "re" => 10, "eq" => 11,
        "ne" => 12, "lt" => 13, "gt" => 14, "fe" => 15,
        "st" => 16, "ad" => 17, "su" => 18, "mu" => 19,
        "di" => 20, "an" => 21, "or" => 22, "xo" => 23,
        "sh" => 24, "zr" => 25, "en" => 26,  _   => 0 ,
    }
}


fn i_pass <'a> (line: String, target: &mut Vec<i32>) {
    let mut v: Vec<&str> = vec!{};
    
    for x in 0..line.len() {
        if x%2 == 0 {
            let (first, rest) = line.split_at(x);
            let (token, unused) = rest.split_at(2);
            println!("Value to be printed:\"{}\" ", token);
            v.push(token);
        }
    }
    for x in 1..v.len()-1{
        println!("Inst: {} \t Op: {}", v[x], opcode_for(v[x]));
        target.push(opcode_for(v[x]));
    }
}
fn pass1(path: &str, target: &mut Vec<i32>, labels: &HashMap<&str, u8>) {
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);
    let mut line_number: usize = 1;

    for line in reader.lines() {
        line_number += 1;
        let this_line = line.unwrap().clone();
        let line_data = this_line.clone();
        let mut first = this_line.split_whitespace();

        match first.next() {
            Some("i") => i_pass(line_data, target), //chars, drop 2, split into 2s, get opcodes, append to target
            Some("r") => target.push(-1), //append 4 dummy u8s to target
            Some("d") => target.push(first.next()
                                     .unwrap()
                                     .parse()
                                     .unwrap_or(0)), //chars, drop 2, parse as u8(how many chars?), append to target
            Some("c") => (), // chars, drop 2, atoi, append target
            Some("s") => (), // chars, drop 2, map atoi, append all to target, 0 terminate?
            Some(":") => (), // chars, drop 2, rest to string, insert into hash with line_number
             _  => (),
        }
    }
}

fn pass2(path: &str, target: &[u8; IMAGE_SIZE], labels: &HashMap<&str, usize>) {
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);
}

const IMAGE_SIZE: usize = 1024;

fn main() {
    let mut target: [i32; IMAGE_SIZE] = [0; IMAGE_SIZE];
    let mut labels: HashMap<&str, u8> = HashMap::new();
    let mut target2: Vec<i32> = vec![];
    let instruction: String = "i: liju....".to_string();

    i_pass(instruction, &mut target2);
    for x in 0..target2.len()-1{
        println!("{}", target2[0]);
    }
}


#[test]
fn it_works(){
    let mut target2: Vec<i32> = vec![];
    let instruction: String = "i liju....".to_string();
    println!("This is the instruction to be printed \"{}\"  ", instruction);
    i_pass(instruction, &mut target2);
    for x in 0..target2.len()-1{
         println!("This is the memory layout of target: {}", target2[x]);
     }
    assert_eq!(target2[0], 1);
}
