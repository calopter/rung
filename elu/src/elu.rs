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

//fn pack_opcode(instructions: &Chars) {}

fn pass1(path: &str, target: &[i32; IMAGE_SIZE], labels: &HashMap<&str, u8>) {
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);
    let mut line_number: usize = 1;

    for line in reader.lines() {
        line_number += 1;
        match line.unwrap().chars().next().unwrap() {
            'i' => (), //chars, drop 2, split into 2s, get opcodes, append to target
            'r' => (), //append 4 dummy u8s to target
            'd' => (), //chars, drop 2, parse as u8(how many chars?), append to target
            'c' => (), // chars, drop 2, atoi, append target
            's' => (), // chars, drop 2, map atoi, append all to target, 0 terminate?
            ':' => (), // chars, drop 2, rest to string, insert into hash with line_number
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
}
