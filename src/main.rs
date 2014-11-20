// Rung: A Rust Ngaro VM


struct VM {
    x: int,
    y: int

}


fn main() {
    let mut vm = VM { x: 5, y: 7 };
    println!("VM State: x: {} , y: {}", vm.x, vm.y);

}
