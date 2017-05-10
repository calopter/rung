extern crate rung_vm;
extern crate elu;

fn main() {
    let vm = rung_vm::make_vm();
    elu::stub();
}
