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
        wram: vec![0; 0x0800],
        ext_ram: vec![0; 0x1FE0],
        backup_ram: vec![0; 0x2000],
        program_rom: rom_data.clone(),
        ppu: ppu,
    };
}

pub fn read_mem_word(mem: &mut CpuMemory, addr: u16) -> u16 {
    let data1 = read_mem(mem, addr) as u16;
    let data2 = read_mem(mem, addr + 1) as u16;
    return (data2 << 8) | data1;
}

pub fn write_mem_word(mem: &mut CpuMemory, addr: u16, data: u16) {
    write_mem(mem, addr, (data & 0xFF) as u8);
    write_mem(mem, addr + 1, ((data & 0xFF00) >> 8) as u8);
}

pub fn read_mem(mem: &mut CpuMemory, addr: u16) -> u8 {
    let mut value = 0u8;
    if addr < 0x0800 {
        value = mem.wram[addr as usize];
    } else if addr < 0x2000 {
        // unused
    } else if addr < 0x2008 || addr == 0x4014 {
        // ppu
        value = ppu::read_io(&mut mem.ppu, addr);
    } else if addr < 0x4000 {
        // unused
    } else if addr < 0x4020 {
        // io
    } else if addr < 0x6000 {
        // ext ram
        value = mem.ext_ram[(addr - 0x4020) as usize];
    } else if addr < 0x8000 {
        // backup rom
        value = mem.backup_ram[(addr - 0x6000) as usize];
    } else if addr <= 0xC000 {
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
    // println!("read {:04X?} value:{:02X}", addr, value);
    return value;
}

pub fn write_mem(mem: &mut CpuMemory, addr: u16, value: u8) {
    // println!("write {:04X} value:{:02X}", addr, value);
    if addr < 0x0800 {
        mem.wram[addr as usize] = value;
    } else if addr < 0x2000 {
        // unused
    } else if addr < 0x2008 || addr == 0x4014 {
        // ppu
        ppu::write_io(&mut mem.ppu, addr, value);
    } else if addr < 0x4000 {
        // unused
    } else if addr < 0x4020 {
    } else if addr < 0x6000 {
        mem.ext_ram[(addr - 0x4020) as usize] = value;
    } else if addr < 0x8000 {
        mem.backup_ram[(addr - 0x6000) as usize] = value;
    } else if addr < 0x8000 {
    } else {
        // program rom
        // read only
    }
}