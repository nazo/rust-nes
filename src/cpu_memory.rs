use super::ppu;

pub struct CpuMemory<'a> {
    pub wram: Vec<u8>,
    pub ext_ram: Vec<u8>,
    pub backup_ram: Vec<u8>,
    pub program_rom: Vec<u8>,
    pub ppu: &'a mut ppu::Ppu,
}

pub fn new_memory<'a>(rom_data: &Vec<u8>, ppu: &'a mut ppu::Ppu) -> CpuMemory<'a> {
    return CpuMemory {
        wram: vec![0; 0x07FF],
        ext_ram: vec![0; 0x1FDF],
        backup_ram: vec![0; 0x1FFF],
        program_rom: rom_data.clone(),
        ppu: ppu,
    };
}

pub fn read_mem_word(mem: &CpuMemory, addr: u16) -> u16 {
    let data1 = read_mem(mem, addr) as u16;
    let data2 = read_mem(mem, addr + 1) as u16;
    return (data2 << 8) | data1;
}

pub fn read_mem(mem: &CpuMemory, addr: u16) -> u8 {
    let mut value = 0u8;
    if addr <= 0x07FF {
        value = mem.wram[addr as usize];
    } else if addr <= 0x1FFF {
        // unused
    } else if addr <= 0x2007 || addr == 0x4014 {
        // ppu
        value = ppu::read_io(&mem.ppu, addr);
    } else if addr <= 0x3FFF {
        // unused
    } else if addr <= 0x401F {
    } else if addr <= 0x5FFF {
    } else if addr <= 0x7FFF {
    } else if addr <= 0xBFFF {
        // program rom
        value = mem.program_rom[(addr - 0x8000) as usize];
    } else {
        // program rom
        if mem.program_rom.len() == 0x4000 {
            value = mem.program_rom[(addr - 0xC000) as usize];
        } else {
            value = mem.program_rom[(addr - 0x8000) as usize];
        }
    }
    println!("read {:04X?} value:{:02X}", addr, value);
    return value;
}

pub fn write_mem(mem: &mut CpuMemory, addr: u16, value: u8) {
    println!("write {:04X} value:{:02X}", addr, value);
    if addr <= 0x07FF {
        mem.wram[addr as usize] = value;
    } else if addr <= 0x1FFF {
        // unused
    } else if addr <= 0x2007 || addr == 0x4014 {
        // ppu
        ppu::write_io(&mut mem.ppu, addr, value);
    } else if addr <= 0x3FFF {
        // unused
    } else if addr <= 0x401F {
    } else if addr <= 0x5FFF {
    } else if addr <= 0x7FFF {
    } else {
        // program rom
        // read only
    }
}