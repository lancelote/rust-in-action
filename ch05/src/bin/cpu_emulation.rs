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
    current_operation: u16,
    registers: [u8; 2],
}

impl CPU {
    fn read_opcode(&self) -> u16 {
        self.current_operation
    }

    #[allow(unused_variables, clippy::identity_op)]
    fn run(&mut self) {
        let opcode = self.read_opcode();

        let c = ((opcode & 0xF000) >> 12) as u8;
        let x = ((opcode & 0x0F00) >> 8) as u8;
        let y = ((opcode & 0x00F0) >> 4) as u8;
        let d = ((opcode & 0x000F) >> 0) as u8;
        let nnn = opcode & 0x0FFF;
        let kk = opcode & 0x00FF;

        match (c, x, y, d) {
            (0x8, _, _, 0x4) => self.add_xy(x, y),
            _ => todo!("opcode {:04x}", opcode),
        }
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        self.registers[x as usize] += self.registers[y as usize];
    }
}

fn main() {
    let mut cpu = CPU {
        current_operation: 0,
        registers: [0; 2],
    };

    cpu.current_operation = 0x8014;
    //                        │││└─ addition operation
    //                        ││└── CPU register 1
    //                        │└─── CPU register 0
    //                        └──── operation involves 2 registers
    cpu.registers[0] = 5;
    cpu.registers[1] = 10;

    cpu.run();

    assert_eq!(cpu.registers[0], 15);
    println!("5 + 10 = {}", cpu.registers[0]);
}
