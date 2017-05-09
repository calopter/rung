// Rung: A Rust Nga VM

type CellInt = i32;

struct VM {
    sp:        usize,
    ip:        usize,
    rp:        usize,

    data:      [CellInt; STACK_DEPTH],
    address:   [CellInt; ADDRESSES],
    memory:    [CellInt; IMAGE_SIZE],
}

impl VM {
    //see Nga.md line 140
    fn load_image(path: &str) -> CellInt {
        0
    }

    fn nop(&self) {
        //avoid dead code elimination?
    }

    fn lit(&mut self) {
        self.sp += 1;
        self.ip += 1;
        self.data[self.sp] = self.memory[self.ip];
    }

    fn dup(&mut self) {
       self.sp += 1;
       self.data[self.sp] = self.data[self.sp - 1];
    }


    fn drop(&mut self) {
        self.data[self.sp] = 0;
        if self.sp == 0 {
            self.ip = IMAGE_SIZE;
        }
    }

    fn swap(&mut self) {
        let a = self.data[self.sp];
        self.data[self.sp] = self.data[self.sp - 1];
        self.data[self.sp - 1] = a;
    }

    fn push(&mut self) {
        self.rp += 1;
        self.address[self.rp] = self.data[self.sp];
        self.drop();
    }

    fn pop(&mut self) {
        self.sp += 1;
        self.data[self.sp] = self.address[self.rp];
        self.rp -= 1;
    }

    fn jump(&mut self) {
        self.ip = (self.data[self.sp] - 1) as usize;
        self.drop();
    }

    fn call(&mut self) {
        self.rp += 1;
        self.address[self.rp] = self.ip as CellInt;
        self.ip = (self.data[self.sp] - 1) as usize;
        self.drop();
    }

    fn ccall(&mut self) {
        let a = self.data[self.sp] as usize;
        self.drop();
        let b = self.data[self.sp];
        self.drop();
        if b != 0 {
            self.rp += 1;
            self.address[self.rp] = self.ip as CellInt;
            self.ip = a - 1;
        }
    }

    fn ret(&mut self) {
        self.ip = self.address[self.rp] as usize;
        self.rp -= 1;
    }

    fn eq(&mut self) {
        self.data[self.sp - 1] = if self.data[self.sp - 1] == self.data[self.sp] {-1} else {0};
        self.drop();
    }

    fn neq(&mut self) {
        self.data[self.sp - 1] = if self.data[self.sp - 1] != self.data[self.sp] {-1} else {0};
        self.drop();
    }

    fn lt(&mut self) {
        self.data[self.sp - 1] = if self.data[self.sp - 1] < self.data[self.sp] {-1} else {0};
        self.drop();
    }

    fn gt(&mut self) {
        self.data[self.sp - 1] = if self.data[self.sp - 1] > self.data[self.sp] {-1} else {0};
        self.drop();
    }

    fn fetch(&mut self) {
        match self.data[self.sp] {
            -1 => self.data[self.sp] = (self.sp - 1) as CellInt,
            -2 => self.data[self.sp] = self.rp as CellInt,
            -3 => self.data[self.sp] = self.memory[self.data[self.sp] as usize],
            _  => self.data[self.sp] = self.memory[self.data[self.sp] as usize],
        }
    }

    fn store(&mut self) {
        self.memory[self.data[self.sp] as usize] = self.data[self.sp - 1];
        self.drop();
        self.drop();
    }

    fn add(&mut self) {
        self.data[self.sp - 1] += self.data[self.sp];
        self.drop();
    }

    fn sub(&mut self) {
        self.data[self.sp - 1] -= self.data[self.sp];
        self.drop();
    }

    fn mul(&mut self) {
        self.data[self.sp - 1] *= self.data[self.sp];
        self.drop();
    }

    fn divmod(&mut self) {
        let (a, b) = (self.data[self.sp], self.data[self.sp - 1]);
        self.data[self.sp] = b / a;
        self.data[self.sp - 1] = b % a;
    }

    fn and(&mut self) {
        self.data[self.sp - 1] = self.data[self.sp] & self.data[self.sp - 1];
        self.drop();
    }

    fn or(&mut self) {
        self.data[self.sp - 1] = self.data[self.sp] | self.data[self.sp - 1];
        self.drop();
    }

    fn xor(&mut self) {
        self.data[self.sp - 1] = self.data[self.sp] ^ self.data[self.sp - 1];
        self.drop();
    }

    //?
    fn shift(&mut self) {
        let (x, y) = (self.data[self.sp], self.data[self.sp - 1]);
        if self.data[self.sp] < 0 {
            self.data[self.sp - 1] = self.data[self.sp - 1] << (self.data[self.sp] * -1);
        } else {
            if x < 0 && y > 0 {
                self.data[self.sp - 1] = x >> y | !(!0 >> y);
            } else {
                self.data[self.sp - 1] = x >> y;
            }
        }
        self.drop();
    }

    fn zret(&mut self) {
        if self.data[self.sp] == 0 {
            self.drop();
            self.ip = self.address[self.rp] as usize;
            self.rp -= 1;
        }
    }

    fn end(&mut self) {
        self.ip = IMAGE_SIZE;
    }

    fn process_opcode(&mut self, opcode: CellInt) {
        match opcode {
            0 => self.nop(),
            1 => self.lit(),
            2 => self.dup(),
            3 => self.drop(),
            4 => self.swap(),
            5 => self.push(),
            6 => self.pop(),
            7 => self.jump(),
            8 => self.call(),
            9 => self.ccall(),
            10 => self.ret(),
            11 => self.eq(),
            12 => self.neq(),
            13 => self.lt(),
            14 => self.gt(),
            15 => self.fetch(),
            16 => self.store(),
            17 => self.add(),
            18 => self.sub(),
            19 => self.mul(),
            20 => self.divmod(),
            21 => self.and(),
            22 => self.or(),
            23 => self.xor(),
            24 => self.shift(),
            25 => self.zret(),
            26 => self.end(),
            _ => self.end(), //throw error
        } 
    }

    fn eval(&mut self) {
        while self.ip < IMAGE_SIZE {
            let opcode = self.memory[self.ip];
            self.process_opcode(opcode);
            self.ip += 1;
            println!("data: {:?}, sp: {}, tos: {}", self.data, self.sp, self.data[self.sp]);
        }
    }
}

//Virtual Machine Parameters
const IMAGE_SIZE:          usize = 5;
const ADDRESSES:           usize = 128;
const STACK_DEPTH:         usize = 8;
//const CELLSIZE:            u32 = 32;
//const NUM_OPS: CellInt = 26;

fn main() {
    let mut vm = VM { sp: 0, ip: 0, rp: 0, 
                      data: [0; STACK_DEPTH], address: [0; ADDRESSES],
                      memory: [1, 1, 1, 2, 4] };
    
    vm.eval();
}
