use super::rom;
use super::cpu_memory;
use log::{info, trace, warn};

mod opcode;

pub struct Cpu {
    pub reg_a: u8,
    pub reg_x: u8,
    pub reg_y: u8,
    pub reg_s: u8,
    pub reg_p: u8,
    pub reg_pc: u16,
}

pub fn new_cpu() -> Cpu {
    return Cpu {
        reg_a: 0,
        reg_x: 0,
        reg_y: 0,
        reg_s: 0,
        reg_p: 0,
        reg_pc: 0x8000,
    };
}

fn reset(cpu: &mut Cpu) {
    cpu.reg_pc = 0x8000;
}

fn fetch_pc_byte(cpu: &mut Cpu, mem: &cpu_memory::CpuMemory) -> u8 {
    let data = cpu_memory::read_mem(mem, cpu.reg_pc);
    cpu.reg_pc += 1;
    return data;
}

fn fetch_pc_word(cpu: &mut Cpu, mem: &cpu_memory::CpuMemory) -> u16 {
    let data = cpu_memory::read_mem_word(mem, cpu.reg_pc);
    cpu.reg_pc += 2;
    return data;
}

fn read_word(data: &[u8], p: u16) -> u16 {
    return ((data[(p + 1) as usize] as u16) << 8) | data[p as usize] as u16;
}

fn write_word(data: &mut Vec<u8>, p: u16, v: u16) {
    data[p as usize] = (v & 0xFF) as u8;
    data[(p + 1) as usize] = ((v & 0xFF00) >> 8) as u8;
}

fn stack_push(cpu: &mut Cpu, mem: &mut cpu_memory::CpuMemory, data: u16) {
    let reg_s = 0x0100 | (cpu.reg_s as u16);
    cpu.reg_s = cpu.reg_s.wrapping_add(1);
    write_word(&mut mem.wram, reg_s, data);
}

fn stack_pop(cpu: &mut Cpu, mem: &cpu_memory::CpuMemory) -> u16 {
    let reg_s = 0x0100 | (cpu.reg_s as u16);
    cpu.reg_s = cpu.reg_s.wrapping_sub(1);
    return read_word(&mem.wram, reg_s);
}

pub fn run(cpu: &mut Cpu, mem: &mut cpu_memory::CpuMemory) {
    println!("pc: {}", cpu.reg_pc);
    let code = fetch_pc_byte(cpu, &mem);
    let op = &opcode::OPCODE_TABLE[code as usize];
    opcode::debug_opcode(code);
    exec_instructions(cpu, mem, op);
}

fn read_by_addressing(cpu: &mut Cpu, mem: &cpu_memory::CpuMemory, op: &opcode::Opcode) -> u16 {
    match op.addressing {
        opcode::ADDRESSING_IMPLIED => {
        }
        opcode::ADDRESSING_IMMEDIATE => {
            return fetch_pc_byte(cpu, &mem) as u16;
        }
        opcode::ADDRESSING_ZEROPAGE => {
            return fetch_pc_byte(cpu, &mem) as u16;
        }
        opcode::ADDRESSING_ZEROPAGE_X => {
            let data = fetch_pc_byte(cpu, &mem) as u16;
            return data + (cpu.reg_x as u16);
        }
        opcode::ADDRESSING_ZEROPAGE_Y => {
            let data = fetch_pc_byte(cpu, &mem) as u16;
            return data + (cpu.reg_y as u16);
        }
        opcode::ADDRESSING_ABSOLUTE => {
            return fetch_pc_word(cpu, &mem);
        }
        opcode::ADDRESSING_ABSOLUTE_X => {
            let data = fetch_pc_word(cpu, &mem);
            return data + (cpu.reg_x as u16);
        }
        opcode::ADDRESSING_ABSOLUTE_Y => {
            let data = fetch_pc_word(cpu, &mem);
            return data + (cpu.reg_y as u16);
        }
        opcode::ADDRESSING_INDIRECT_X => {
            let data = fetch_pc_byte(cpu, &mem) as u16;
            panic!("");
            return data; // todo
        }
        opcode::ADDRESSING_INDIRECT_Y => {
            let data = fetch_pc_byte(cpu, &mem) as u16;
            panic!("");
            return data; // todo
        }
        opcode::ADDRESSING_INDIRECT => {
            let data = fetch_pc_word(cpu, &mem) as u16;
            panic!("");
            return data; // todo
        }
        opcode::ADDRESSING_ACCUMULATOR => {
            return cpu.reg_a as u16;
        }
        opcode::ADDRESSING_RELATIVE => {
            return fetch_pc_byte(cpu, &mem) as u16;
        }
        _ => {
        }
    }
    return 0;
}

fn exec_instructions(cpu: &mut Cpu, mem: &mut cpu_memory::CpuMemory, op: &opcode::Opcode) {
    let mut data = read_by_addressing(cpu, mem, op);
    let relative = (data as u8) as i8;
    println!("data: {:04X}", data);
    match op.code {
        opcode::OPCODE_LDA => {
            if op.addressing != opcode::ADDRESSING_IMMEDIATE {
                data = cpu_memory::read_mem(mem, data as u16) as u16;
            }
            cpu.reg_a = data as u8;
            cpu.reg_p = (cpu.reg_p & 0x7D) | (cpu.reg_a & 0x80) | (if cpu.reg_a == 0 { 2 } else { 0 });
        }
        opcode::OPCODE_LDX => {
            if op.addressing != opcode::ADDRESSING_IMMEDIATE {
                data = cpu_memory::read_mem(mem, data as u16) as u16;
            }
            cpu.reg_x = data as u8;
            cpu.reg_p = (cpu.reg_p & 0x7D) | (cpu.reg_x & 0x80) | (if cpu.reg_x == 0 { 2 } else { 0 });
        }
        opcode::OPCODE_LDY => {
            if op.addressing != opcode::ADDRESSING_IMMEDIATE {
                data = cpu_memory::read_mem(mem, data as u16) as u16;
            }
            cpu.reg_y = data as u8;
            cpu.reg_p = (cpu.reg_p & 0x7D) | (cpu.reg_y & 0x80) | (if cpu.reg_y == 0 { 2 } else { 0 });
        }
        opcode::OPCODE_STA => {
            cpu_memory::write_mem(mem, data, cpu.reg_a);
        }
        opcode::OPCODE_STX => {
            cpu_memory::write_mem(mem, data, cpu.reg_x);
        }
        opcode::OPCODE_STY => {
            cpu_memory::write_mem(mem, data, cpu.reg_y);
        }
        opcode::OPCODE_TAX => {
            cpu.reg_x = cpu.reg_a;
            cpu.reg_p = (cpu.reg_p & 0x7D) | (cpu.reg_x & 0x80) | (if cpu.reg_x == 0 { 2 } else { 0 });
        }
        opcode::OPCODE_TAY => {
            cpu.reg_y = cpu.reg_a;
            cpu.reg_p = (cpu.reg_p & 0x7D) | (cpu.reg_y & 0x80) | (if cpu.reg_y == 0 { 2 } else { 0 });
        }
        opcode::OPCODE_TSX => {
            cpu.reg_x = cpu.reg_s;
            cpu.reg_p = (cpu.reg_p & 0x7D) | (cpu.reg_x & 0x80) | (if cpu.reg_x == 0 { 2 } else { 0 });
        }
        opcode::OPCODE_TXA => {
            cpu.reg_a = cpu.reg_x;
            cpu.reg_p = (cpu.reg_p & 0x7D) | (cpu.reg_a & 0x80) | (if cpu.reg_a == 0 { 2 } else { 0 });
        }
        opcode::OPCODE_TXS => {
            cpu.reg_s = cpu.reg_x;
            cpu.reg_p = (cpu.reg_p & 0x7D) | (cpu.reg_s & 0x80) | (if cpu.reg_s == 0 { 2 } else { 0 });
        }
        opcode::OPCODE_TYA => {
            cpu.reg_a = cpu.reg_y;
            cpu.reg_p = (cpu.reg_p & 0x7D) | (cpu.reg_a & 0x80) | (if cpu.reg_a == 0 { 2 } else { 0 });
        }
        opcode::OPCODE_ADC => {
            panic!("not implemented");
        }
        opcode::OPCODE_AND => {
            panic!("not implemented");
        }
        opcode::OPCODE_ASL => {
            panic!("not implemented");
        }
        opcode::OPCODE_BIT => {
            let test = cpu_memory::read_mem(mem, data as u16) as u8;
            let zflag = (if (cpu.reg_a & test) == 0 { 2 } else { 0 }) as u8;
            cpu.reg_p = (cpu.reg_p & 0x3D) | (test & 0xC0) | zflag;
        }
        opcode::OPCODE_CMP => {
            panic!("not implemented");
        }
        opcode::OPCODE_CPX => {
            panic!("not implemented");
        }
        opcode::OPCODE_CPY => {
            panic!("not implemented");
        }
        opcode::OPCODE_INC => {
            let mut value = cpu_memory::read_mem(mem, data as u16) as u8;
            value = value.wrapping_add(1);
            cpu_memory::write_mem(mem, data, value);
            cpu.reg_p = (cpu.reg_p & 0x7D) | (value & 0x80) | (if value == 0 { 2 } else { 0 });
        }
        opcode::OPCODE_DEC => {
            let mut value = cpu_memory::read_mem(mem, data as u16) as u8;
            value = value.wrapping_sub(1);
            cpu_memory::write_mem(mem, data, value);
            cpu.reg_p = (cpu.reg_p & 0x7D) | (value & 0x80) | (if value == 0 { 2 } else { 0 });
        }
        opcode::OPCODE_DEX => {
            cpu.reg_x = cpu.reg_x.wrapping_sub(1);
            cpu.reg_p = (cpu.reg_p & 0x7D) | (cpu.reg_x & 0x80) | (if cpu.reg_x == 0 { 2 } else { 0 });
        }
        opcode::OPCODE_DEY => {
            cpu.reg_y = cpu.reg_y.wrapping_sub(1);
            cpu.reg_p = (cpu.reg_p & 0x7D) | (cpu.reg_y & 0x80) | (if cpu.reg_y == 0 { 2 } else { 0 });
        }
        opcode::OPCODE_EOR => {
            if op.addressing != opcode::ADDRESSING_IMMEDIATE {
                data = cpu_memory::read_mem(mem, data as u16) as u16;
            }
            cpu.reg_a = cpu.reg_a ^ (data as u8);
            cpu.reg_p = (cpu.reg_p & 0x7D) | (cpu.reg_a & 0x80) | (if cpu.reg_a == 0 { 2 } else { 0 });
        }
        opcode::OPCODE_INX => {
            cpu.reg_x = cpu.reg_x.wrapping_add(1);
            cpu.reg_p = (cpu.reg_p & 0x7D) | (cpu.reg_x & 0x80) | (if cpu.reg_x == 0 { 2 } else { 0 });
        }
        opcode::OPCODE_INY => {
            cpu.reg_y = cpu.reg_y.wrapping_add(1);
            cpu.reg_p = (cpu.reg_p & 0x7D) | (cpu.reg_y & 0x80) | (if cpu.reg_y == 0 { 2 } else { 0 });
        }
        opcode::OPCODE_LSR => {
            panic!("not implemented");
        }
        opcode::OPCODE_ORA => {
            panic!("not implemented");
        }
        opcode::OPCODE_ROL => {
            panic!("not implemented");
        }
        opcode::OPCODE_ROR => {
            panic!("not implemented");
        }
        opcode::OPCODE_SBC => {
            panic!("not implemented");
        }
        opcode::OPCODE_PHA => {
            panic!("not implemented");
        }
        opcode::OPCODE_PHP => {
            panic!("not implemented");
        }
        opcode::OPCODE_BEQ => {
            if (cpu.reg_p & 0x02) != 0 {
                cpu.reg_pc = (cpu.reg_pc as i32 + relative as i32) as u16;
            }
        }
        opcode::OPCODE_BNE => {
            if (cpu.reg_p & 0x02) == 0 {
                cpu.reg_pc = (cpu.reg_pc as i32 + relative as i32) as u16;
            }
        }
        opcode::OPCODE_BMI => {
            if (cpu.reg_p & 0x80) != 0 {
                cpu.reg_pc = (cpu.reg_pc as i32 + relative as i32) as u16;
            }
        }
        opcode::OPCODE_BPL => {
            if (cpu.reg_p & 0x80) == 0 {
                cpu.reg_pc = (cpu.reg_pc as i32 + relative as i32) as u16;
            }
        }
        opcode::OPCODE_BVS => {
            if (cpu.reg_p & 0x40) != 0 {
                cpu.reg_pc = (cpu.reg_pc as i32 + relative as i32) as u16;
            }
        }
        opcode::OPCODE_BVC => {
            if (cpu.reg_p & 0x40) == 0 {
                cpu.reg_pc = (cpu.reg_pc as i32 + relative as i32) as u16;
            }
        }
        opcode::OPCODE_BCS => {
            if (cpu.reg_p & 0x01) != 0 {
                cpu.reg_pc = (cpu.reg_pc as i32 + relative as i32) as u16;
            }
        }
        opcode::OPCODE_BCC => {
            if (cpu.reg_p & 0x01) == 0 {
                cpu.reg_pc = (cpu.reg_pc as i32 + relative as i32) as u16;
            }
        }
        opcode::OPCODE_JMP => {
            cpu.reg_pc = data;
        }
        opcode::OPCODE_JSR => {
            let reg_pc = cpu.reg_pc;
            stack_push(cpu, mem, reg_pc);
            cpu.reg_pc = data;
        }
        opcode::OPCODE_RTS => {
            let addr = stack_pop(cpu, mem) + 1;
            cpu.reg_pc = addr;
        }
        opcode::OPCODE_SEI => {
            cpu.reg_p = cpu.reg_p | 0x04;
        }
        opcode::OPCODE_CLI => {
            cpu.reg_p = cpu.reg_p & 0xFB;
        }
        opcode::OPCODE_CLD => {
            cpu.reg_p = cpu.reg_p & 0xF7;
        }
        opcode::OPCODE_CLV => {
            cpu.reg_p = cpu.reg_p & 0xBF;
        }
        opcode::OPCODE_SEC => {
            cpu.reg_p = cpu.reg_p | 0x01;
        }
        opcode::OPCODE_SED => {
            cpu.reg_p = cpu.reg_p | 0x08;
        }
        opcode::OPCODE_CLC => {
            cpu.reg_p = cpu.reg_p & 0xFE;
        }
        opcode::OPCODE_BRK => {
            cpu.reg_p = cpu.reg_p | 0x10;
        }
        opcode::OPCODE_NOP => {
            // nop
        }
        _ => {
            panic!("not implemented");
        }
    }
}