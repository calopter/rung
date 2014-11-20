// Rung: A Rust Ngaro VM


struct VM {
    x: int,
    y: int,
    zed: Cell,

}

struct Cell (u32);

fn main() {
    let mut vm = VM { x: 5, y: 7, zed: Cell(23) };
    vm.x = 14;
    let Cell(zed) = vm.zed;
    println!("VM State: x: {} , y: {}, zed: {} ", vm.x, vm.y, zed  );

}
