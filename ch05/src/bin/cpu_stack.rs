// Bytes and Nibbles
//
// 0x0000
//   ││└┴─ low byte
//   └┴─── high byte
// 0x0000
//    └─┴─ low nibbles
// 0x0000
//   └─┴── high nibbles
//
// Variables
//
// 0x0000
//      └─ n - number of bytes
// 0x0000
//    └─── x - CPU register
// 0x0000
//     └── y - CPU register
// 0x0000
//   └──── c - opcode group
// 0x0000
//      └─ d - opcode subgroup (same as `n` but different context)
// 0x0000
//     └┴─ kk - integer
// 0x0000
//    └─┴─ nnn - memory address
//
// Opcode Forms
//
// 0x73EE - add 238 (0xEE) to register 3
//   ││└┴─ (kk) argument
//   │└─── (x) register
//   └──── (c) opcode group
//
// 0x1200 - jump to memory address (0x200)
//   │└─┴─ (nnn) address
//   └──── (c) opcode group
//
// 0x8231 - bitwise OR with registers x and y, store result in x
//   │││└─ (d) opcode subtype
//   ││└── (y) register
//   │└─── (x) register
//   └──── (c) opcode group

extern crate core;

#[allow(clippy::upper_case_acronyms)]
struct CPU {
    registers: [u8; 16],
    position_in_memory: usize,
    memory: [u8; 0x1000], // 4096 bytes
    stack: [u16; 16],     // after 16 nested calls - stack overflow
    stack_pointer: usize,
}

impl CPU {
    fn read_opcode(&self) -> u16 {
        let p = self.position_in_memory;
        let op_byte1 = self.memory[p] as u16;
        let op_byte2 = self.memory[p + 1] as u16;

        op_byte1 << 8 | op_byte2
    }

    #[allow(unused_variables, clippy::identity_op)]
    fn run(&mut self) {
        loop {
            let opcode = self.read_opcode();
            self.position_in_memory += 2;

            let c = ((opcode & 0xF000) >> 12) as u8;
            let x = ((opcode & 0x0F00) >> 8) as u8;
            let y = ((opcode & 0x00F0) >> 4) as u8;
            let d = ((opcode & 0x000F) >> 0) as u8;
            let nnn = opcode & 0x0FFF;
            let kk = opcode & 0x00FF;

            match (c, x, y, d) {
                (0, 0, 0, 0) => {
                    return;
                }
                (0, 0, 0xE, 0xE) => self.ret(),
                (0x2, _, _, _) => self.call(nnn),
                (0x8, _, _, 0x4) => self.add_xy(x, y),
                _ => todo!("opcode {:04x}", opcode),
            }
        }
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];

        let (val, overflow) = arg1.overflowing_add(arg2);
        self.registers[x as usize] = val;

        // last registry as carry flag
        if overflow {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
    }

    fn call(&mut self, addr: u16) {
        let sp = self.stack_pointer;
        let stack = &mut self.stack;

        if sp > stack.len() {
            panic!("Stack overflow!");
        }

        stack[sp] = self.position_in_memory as u16;
        self.stack_pointer += 1;
        self.position_in_memory = addr as usize;
    }

    fn ret(&mut self) {
        if self.stack_pointer == 0 {
            panic!("Stack underflow");
        }

        self.stack_pointer -= 1;
        let call_addr = self.stack[self.stack_pointer];
        self.position_in_memory = call_addr as usize;
    }
}

fn main() {
    let mut cpu = CPU {
        registers: [0; 16],
        memory: [0; 4096],
        position_in_memory: 0,
        stack: [0; 16],
        stack_pointer: 0,
    };

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;

    let mem = &mut cpu.memory;

    // call the function at 0x100
    mem[0x000] = 0x21;
    mem[0x001] = 0x00;
    // call the function at 0x100 once again
    mem[0x002] = 0x21;
    mem[0x003] = 0x00;
    // halt
    mem[0x004] = 0x00;
    mem[0x005] = 0x00;

    // add register 1 to register 0
    mem[0x100] = 0x80;
    mem[0x101] = 0x14;
    // add register 1 to register 0
    mem[0x102] = 0x80;
    mem[0x103] = 0x14;
    // function return
    mem[0x104] = 0x00;
    mem[0x105] = 0xEE;

    cpu.run();

    assert_eq!(cpu.registers[0], 45);
    println!("5 + (10 * 2) + (10 * 2) = {}", cpu.registers[0]);
}
