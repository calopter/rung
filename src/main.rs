// Rung: A Rust Ngaro VM

//Rusty stuff
#![feature(tuple_indexing)]
use std::default::Default;
use std::fmt;


struct VM {
    sp: Cell,
    ip: Cell,
    rsp: Cell,
    data: [Cell, ..STACK_DEPTH],
    address: [Cell, ..ADDRESSES],
    ports: [Cell, ..PORTS],
    image: [Cell, ..IMAGE_SIZE],
}
impl Default for VM {
    fn default() -> VM {
        VM {
            sp: Cell(0),
            ip: Cell(128),
            rsp: Cell(256),
            data : [NOP, ..STACK_DEPTH],
            address : [NOP, ..ADDRESSES],
            ports: [NOP, ..PORTS],
            image: [NOP, ..IMAGE_SIZE],
        }
    }
}
impl fmt:: Show for VM {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.sp.0, self.ip.0)
    }
}

struct Cell (u32);

//Virtual Machine Parameters
const STACK_DEPTH:           uint =  128;
const IMAGE_SIZE:            uint =  1000000;
const ADDRESSES:             uint =  1024;
const PORTS:                 uint =  12;
const MAX_FILE_NAME:         uint =  1024;
const MAX_REQUEST_LENGTH:    uint =  1024;
const MAX_OPEN_FILES:        uint =  8;
const LOCAL:        &'static str  = "retroImage" ;
const CELLSIZE:              uint = 32;

//Ngaro VM Opcodes
const NOP:       Cell = Cell(0);
const LIT:       Cell = Cell(1);
const DUP:       Cell = Cell(2);
const DROP:      Cell = Cell(3);
const SWAP:      Cell = Cell(4);
const PUSH:      Cell = Cell(5);
const POP:       Cell = Cell(6);
const LOOP:      Cell = Cell(7);
const JUMP:      Cell = Cell(8);
const RETURN:    Cell = Cell(9);
const GT_JUMP:   Cell = Cell(10);
const LT_JUMP:   Cell = Cell(11);
const NE_JUMP:   Cell = Cell(12);
const EQ_JUMP:   Cell = Cell(13);
const FETCH:     Cell = Cell(14);
const STORE:     Cell = Cell(15);
const ADD:       Cell = Cell(16);
const SUB:       Cell = Cell(17);
const MUL:       Cell = Cell(18);
const DIVMOD:    Cell = Cell(19);
const AND:       Cell = Cell(20);
const OR:        Cell = Cell(21);
const XOR:       Cell = Cell(22);
const SHL:       Cell = Cell(23);
const ZERO_EXIT: Cell = Cell(24);
const INC:       Cell = Cell(25);
const DEC:       Cell = Cell(26);
const IN:        Cell = Cell(27);
const OUT:       Cell = Cell(28);
const WAIT:      Cell = Cell(29);

const NUM_OPS:   Cell = Cell(WAIT.0 + 1) ;



fn main() {
    let mut vm = box VM { ..Default::default() };
    vm.ip = ZERO_EXIT;
    let (Cell(rsp), Cell(sp), Cell(ip)) = (vm.rsp, vm.sp, vm.ip);
    println!("VM State: x: {} , y: {}, rsp: {} ", sp, ip, rsp  );
    println!("VM (Formatted) : {}", vm);

}
