// Rung: A Rust Ngaro VM

//Rusty stuff
#![feature(tuple_indexing)]



struct VM {
    sp: Cell,
    ip: Cell,
    rsp: Cell,
    data: [Cell ..STACK_DEPTH],
}

struct Cell (u32);

//Virtual Machine Parameters
const STACK_DEPTH:           int =  128;
const IMAGE_SIZE:            int =  1000000;
const ADDRESSES:             int =  1024;
const PORTS:                 int =  12;
const MAX_FILE_NAME:         int =  1024;
const MAX_REQUEST_LENGTH:    int =  1024;
const MAX_OPEN_FILES:        int =  8;
const LOCAL:        &'static str = "retroImage" ;
const CELLSIZE:              int = 32;

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
    let mut vm = VM {
                      sp: NOP,
                      ip:  Cell(0),
                      rsp: Cell(23),
                    };
    vm.ip = ZERO_EXIT;
    let (Cell(rsp), Cell(sp), Cell(ip)) = (vm.rsp, vm.sp, vm.ip);
    println!("VM State: x: {} , y: {}, rsp: {} ", sp, ip, rsp  );

}
