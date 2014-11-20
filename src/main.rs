// Rung: A Rust Ngaro VM


struct VM {
    x: int,
    y: int,
    sp: Cell,
    ip: Cell,
    rsp: Cell,
}

struct Cell (u32);

enum Op {
    Nop,
    Lit,
    Dup,
    Drop,
    Swap,
    Push,
    Pop,
    Loop,
    Jump,
    Return,
    GtJump,
    LtJump,
    NeJump,
    EqJump,
    Fetch,
    Store,
    Add,
    Sub,
    Mul,
    DivMod,
    And,
    Or,
    Xor,
    Shl,
    Shr,
    ZeroExit,
    Inc,
    Dec,
    In,
    Out,
    Wait

}

fn main() {
    let mut vm = VM { x: 5,
                      y: 7,
                      sp: Cell(0),
                      ip:  Cell(0),
                      rsp: Cell(23),
                    };
    vm.x = 14;
    let Cell(rsp) = vm.rsp;
    println!("VM State: x: {} , y: {}, rsp: {} ", vm.x, vm.y, rsp  );

}
