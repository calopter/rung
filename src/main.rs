// Rung: A Rust Nga VM

#![feature(tuple_indexing)]

use std::default::Default;
use std::fmt;

type CellInt = i32;

struct VM {
    sp:        usize,
    ip:        usize,
    rp:        usize,

    tos:       CellInt,
    nos:       CellInt,
    tors:      CellInt,

    data:      [CellInt; STACK_DEPTH],
    address:   [CellInt; ADDRESSES],

    ports:     [CellInt; PORTS],

    memory:    [CellInt; IMAGE_SIZE],
    max_rsp:   u32,
    max_sp:    u32,
    filename:  String,
    request:   String,
}

impl VM {
    //see Nga.md line 140
    fn load_image(path: &str) -> CellInt {
        0
    }

    //516
    fn process_opcode(&self, opcode: VM_opcode) {} 

    // instructions //
    //              //
    //              //

    fn nop() {
        //avoid dead code elimination?
    }

    fn lit(&mut self) {
        self.sp += 1;
        self.ip += 1;
        self.tos = self.memory[self.ip];
    }

    fn dup(&mut self) {
       self.sp += 1;
       self.data[self.sp] = self.nos;
    }


    fn drop(&mut self) {
        self.data[self.sp] = 0;
        if self.sp - 1 < 0 {
            self.ip = IMAGE_SIZE;
        }
    }

    fn swap(&mut self) {
        let a = self.tos;
        self.tos = self.nos;
        self.nos = a;
    }

    fn push(&mut self) {
        self.rp += 1;
        self.tors = self.tos;
        self.drop();
    }

    fn pop(&mut self) {
        self.sp += 1;
        self.tos = self.tors;
        self.rp -= 1;
    }

    fn jump(&mut self) {
        self.ip = (self.tos - 1) as usize;
        self.drop();
    }

    fn call(&mut self) {
        self.rp += 1;
        self.tors = self.ip as CellInt;
        self.ip = (self.tos - 1) as usize;
        self.drop();
    }

    fn ccall(&mut self) {
        let a = self.tos as usize;
        self.drop();
        let b = self.tos;
        self.drop();
        if b != 0 {
            self.rp += 1;
            self.tors = self.ip as CellInt;
            self.ip = a - 1;
        }
    }

    fn ret(&mut self) {
        self.ip = self.tors as usize;
        self.rp -= 1;
    }

    fn eq(&mut self) {

    }

    fn neq(&mut self) {

    }

    fn lt(&mut self) {

    }

    fn gt(&mut self) {

    }

    fn fetch(&mut self) {

    }

    fn store(&mut self) {

    }

    fn add(&mut self) {

    }

    fn sub(&mut self) {

    }

    fn mul(&mut self) {

    }

    fn divmod(&mut self) {

    }

    fn and(&mut self) {

    }

    fn or(&mut self) {

    }

    fn xor(&mut self) {

    }

    fn shift(&mut self) {

    }

    fn zret(&mut self) {

    }

    fn end(&mut self) {

    }


}

impl Default for VM {
    fn default() -> VM {
        VM {
            sp:        0,
            ip:        0,
            rp:        0,
            tos:       0,
            nos:       0,
            tors:      0,
            data:      [NOP as CellInt; STACK_DEPTH],
            address:   [NOP as CellInt; ADDRESSES],
            memory:    [NOP as CellInt; IMAGE_SIZE],
            ports:     [NOP as CellInt; PORTS],
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

struct Image([Cell; IMAGE_SIZE]);

#[derive (Clone, Copy)]
enum Cell {
    Int(CellInt), Opcode(VM_opcode)
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write! (f, "{}", self)
    }
}

//Virtual Machine Parameters
const IMAGE_SIZE:          usize =  524288;
const ADDRESSES:           usize =  128;
const STACK_DEPTH:         usize =  32;

const PORTS:               usize =  12;
const MAX_FILE_NAME:       u32 =  1024;
const MAX_REQUEST_LENGTH:  u32 =  1024;
const MAX_OPEN_FILES:      u32 =  8;
const LOCAL:       &'static str = "retroImage" ;
const CELLSIZE:            u32 = 32;

//Nga VM Opcodes
#[derive (Clone, Copy, Debug)]
enum VM_opcode {
    NOP,
    LIT,
    DUP,
    DROP,
    SWAP,
    PUSH,
    POP,
    JUMP,
    CALL,
    CCALL,
    RETURN,
    EQ,
    NEQ,
    LT,
    GT,
    FETCH,
    STORE,
    ADD,
    SUB,
    MUL,
    DIVMOD,
    AND,
    OR,
    XOR,
    SHIFT,
    ZRET,
    END,
}

impl fmt::Display for VM_opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write! (f, "{}", self)
    }
}

fn from_u8(n: u8) -> Option<VM_opcode> {
    if n >= NOP as u8 && n <= END as u8 {
        Some(unsafe {std::mem::transmute(n)})
    } else {
        None
    }
}

use VM_opcode::*;


const NUM_OPS: CellInt = (END as i32) + 1 ;

fn main() {
    let mut vm: VM = Default::default(); //syntax for overide { ..Default::default() };
 //   let (Cell(rsp), Cell(sp), Cell(ip)) = (vm.rp, vm.sp, vm.ip);
    println!("VM State: x: {} , y: {}, rp: {} ", vm.sp, vm.ip, vm.rp);
    println!("VM (Formatted) : {}", vm);
    //let image = &vm.image.0;
    //println!("Printed from the Image: {:08x}", image[5].0);

}
