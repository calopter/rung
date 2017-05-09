// Rung: A Rust Nga VM

#![feature(tuple_indexing)]

use std::default::Default;
use std::fmt;

type CellInt = i32;

struct VM {
    sp:        CellInt,
    ip:        CellInt,
    rsp:       CellInt,
    data:      [Cell; STACK_DEPTH],
    address:   [Cell; ADDRESSES],

    ports:     [Cell; PORTS],

    image:     Box<Image>,
    stats:     [u32; 30],
    max_rsp:   u32,
    max_sp:    u32,
    filename:  String,
    request:   String,
}

struct Image([Cell; IMAGE_SIZE]);

impl Default for VM {
    fn default() -> VM {
        VM {
            sp:        0,
            ip:        128,
            rsp:       256,
            data:      [Cell(NOP); STACK_DEPTH],
            address:   [Cell(NOP); ADDRESSES],
            ports:     [Cell(NOP); PORTS],
            image:     Box::new(Image([INIT; IMAGE_SIZE])),
            stats:     [0; 30],
            max_rsp:   ADDRESSES as u32,
            max_sp:    STACK_DEPTH as u32,
            filename:  String::new(),
            request:   String::new(),

        }
    }
}

impl fmt::Display for VM {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        s.push_str("Stack: ");
        s.push_str(self.ip.to_string().as_str());
        write!(f,"{}",s)
    }

}

#[derive (Clone, Copy)]
struct Cell (CellInt);

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write! (f, "{}", self.0)
    }
}

//Virtual Machine Parameters
const STACK_DEPTH:         usize =  128;
const IMAGE_SIZE:          usize =  1000000;
const ADDRESSES:           usize =  1024;
const PORTS:               usize =  12;
const MAX_FILE_NAME:       u32 =  1024;
const MAX_REQUEST_LENGTH:  u32 =  1024;
const MAX_OPEN_FILES:      u32 =  8;
const LOCAL:       &'static str = "retroImage" ;
const CELLSIZE:            u32 = 32;

//Ngaro VM Opcodes
const NOP:      CellInt = 0;
const LIT:      CellInt = 1;
const DUP:      CellInt = 2;
const DROP:     CellInt = 3;
const SWAP:     CellInt = 4;
const PUSH:     CellInt = 5;
const POP:      CellInt = 6;
const LOOP:     CellInt = 7;
const JUMP:     CellInt = 8;
const RETURN:   CellInt = 9;
const GT_JUMP:  CellInt = 10;
const LT_JUMP:  CellInt = 11;
const NE_JUMP:  CellInt = 12;
const EQ_JUMP:  CellInt = 13;
const FETCH:    CellInt = 14;
const STORE:    CellInt = 15;
const ADD:      CellInt = 16;
const SUB:      CellInt = 17;
const MUL:      CellInt = 18;
const DIVMOD:   CellInt = 19;
const AND:      CellInt = 20;
const OR:       CellInt = 21;
const XOR:      CellInt = 22;
const SHL:      CellInt = 23;
const ZERO_EXIT:CellInt = 24;
const INC:      CellInt = 25;
const DEC:      CellInt = 26;
const IN:       CellInt = 27;
const OUT:      CellInt = 28;
const WAIT:     CellInt = 29;

const NEWCONST: i32 = -5 ;

//Clearing constant
const INIT:      Cell = Cell(0x0000DEAD);
const NUM_OPS:   CellInt  = WAIT + 1 ;

fn main() {
    let mut vm = VM { ..Default::default() };
 //   let (Cell(rsp), Cell(sp), Cell(ip)) = (vm.rsp, vm.sp, vm.ip);
    println!("VM State: x: {} , y: {}, rsp: {} ", vm.sp, vm.ip, vm.rsp  );
    println!("VM (Formatted) : {}", vm);
    let image = &vm.image.0;
    println!("Printed from the Image: {:08x}", image[5].0);

}
