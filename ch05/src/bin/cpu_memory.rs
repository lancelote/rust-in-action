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

#[allow(clippy::upper_case_acronyms)]
struct CPU {
    registers: [u8; 16],
    position_in_memory: usize,
    memory: [u8; 0x1000], // 4096 bytes
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
                } // exit opcode is 0x0000
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
}

fn main() {
    let mut cpu = CPU {
        registers: [0; 16],
        memory: [0; 4096],
        position_in_memory: 0,
    };

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;
    cpu.registers[2] = 10;
    cpu.registers[3] = 10;

    let mem = &mut cpu.memory;
    mem[0] = 0x80; // adds register 1 to register 0
    mem[1] = 0x14;
    mem[2] = 0x80; // adds register 2 to register 0
    mem[3] = 0x24;
    mem[4] = 0x80; // adds register 3 to register 0
    mem[5] = 0x34;

    cpu.run();

    assert_eq!(cpu.registers[0], 35);
    println!("5 + 10 + 10 + 10 = {}", cpu.registers[0]);
}
