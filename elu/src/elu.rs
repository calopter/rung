// elu: a clone of muri, assembler for rung/the Nga VM

extern crate byteorder;

use byteorder::{LittleEndian, WriteBytesExt};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;

pub fn save(target: &[i32; 4], path: &str) {
    let mut file = File::create(path).unwrap();
    
    for i in target.iter() {
        file.write_i32::<LittleEndian>(*i);
    }
}

fn read_line(path: &str) -> String {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    reader.lines().next().unwrap().unwrap()
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

fn main() {
    let mut labels: HashMap<&str, i32> = HashMap::new();
}
