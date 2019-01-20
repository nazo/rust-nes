mod palette;

pub struct Ppu {
    vram: Vec<u8>,
    oam: Vec<u8>,
    vram_address: u16,
    vram_write_counter: u8,
    h_scroll: u8,
    v_scroll: u8,
    scroll_write_counter: u8,
    oam_address: u8,
    reg_controller: u8,
    reg_mask: u8,
    reg_status: u8,
}

pub fn new_ppu(rom_data: &Vec<u8>) -> Ppu {
    let mut vram = vec![0; 0xFFFF];
    for i in 0..rom_data.len() {
        vram[i] = rom_data[i];
    }
    return Ppu {
        vram: vram,
        oam: vec![0; 256],
        vram_write_counter: 0,
        scroll_write_counter: 0,
        vram_address: 0,
        h_scroll: 0,
        v_scroll: 0,
        oam_address: 0,
        reg_controller: 0,
        reg_mask: 0,
        reg_status: 0,
    };
}

pub fn read_io(ppu: &mut Ppu, addr: u16) -> u8 {
    match addr {
        0x2000 => {
            // ppu controller
        }
        0x2001 => {
            // ppu mask
        }
        0x2002 => {
            // ppu status
            let status = ppu.reg_status;
            ppu.reg_status = ppu.reg_status & 0x7F;
            ppu.scroll_write_counter = 0;
            ppu.vram_write_counter = 0;
            return status;
        }
        0x2003 => {
            // oam address
        }
        0x2004 => {
            // oam access
        }
        0x2005 => {
            // scroll
        }
        0x2006 => {
            // vram address
        }
        0x2007 => {
            // vram access
            return ppu.vram[ppu.vram_address as usize];
        }
        0x4014 => {
            // oam dma
        }
        _ => {
        }
    }
    return 0;
}

pub fn write_io(ppu: &mut Ppu, addr: u16, value: u8) {
    match addr {
        0x2000 => {
            // ppu controller
            ppu.reg_controller = value;
        }
        0x2001 => {
            // ppu mask
            ppu.reg_mask = value;
        }
        0x2002 => {
            // ppu status
        }
        0x2003 => {
            // oam address
            ppu.oam_address = value;
        }
        0x2004 => {
            // oam access
            ppu.oam[ppu.oam_address as usize] = value;
            // println!("oam address {:04X} = {:02X}", ppu.oam_address, value);
        }
        0x2005 => {
            // scroll
            if ppu.scroll_write_counter == 0 {
                ppu.h_scroll = value;
            } else {
                ppu.v_scroll = value;
            }
            ppu.scroll_write_counter = ppu.scroll_write_counter ^ 1;
        }
        0x2006 => {
            // vram address
            let shift: u8 = (1 - ppu.vram_write_counter) * 8;
            if ppu.vram_write_counter == 0 {
                ppu.vram_address = 0;
            }
            ppu.vram_address = ppu.vram_address | ((value as u16) << shift);
            ppu.vram_write_counter = ppu.vram_write_counter ^ 1;
        }
        0x2007 => {
            // vram access
            ppu.vram[ppu.vram_address as usize] = value;
            // println!("vram address {:04X} = {:02X}", ppu.vram_address, value);
            ppu.vram_address += 1;
        }
        0x4014 => {
            // oam dma
            println!("oam dma");
        }
        _ => {
        }
    }
}

const ADDR_PATTERN0: u16 = 0x0000;
const ADDR_PATTERN1: u16 = 0x1000;
const ADDR_BG0: u16 = 0x2000;
const ADDR_BG1: u16 = 0x2400;
const ADDR_BG2: u16 = 0x2800;
const ADDR_BG3: u16 = 0x2C00;

#[inline(always)]
fn put_pixel(canvas: &mut Vec<u8>, x: i32, y: i32, r: u8, g: u8, b: u8) {
    if 0 > x || x >= 256 {
        return;
    }
    if 0 > y || y >= 240 {
        return;
    }

    let offset = (y*(256*3) + x * 3) as usize;
    canvas[offset] = r;
    canvas[offset + 1] = g;
    canvas[offset + 2] = b;
}

#[inline(always)]
fn get_palette(ppu: &Ppu, palette_num: u8, offset: u8) -> (u8, u8, u8) {
    let mut address = (0x3F00 + (offset as u16) + (palette_num as u16)) as usize;
    if palette_num == 0 {
        address = 0x3F00;
    }
    let palette_color = ((ppu.vram[address] & 0x3F) * 3) as usize;
    return (palette::PALETTE_TABLE[palette_color], palette::PALETTE_TABLE[palette_color + 1], palette::PALETTE_TABLE[palette_color + 2]);
}

#[inline(always)]
fn put_tile(canvas: &mut Vec<u8>, ppu: &Ppu, base_x: i32, base_y: i32, base_addr: u16, chrnum: u8, palette_offset: u8) {
    for y in 0..8 {
        let addr = base_addr + ((chrnum as u16) * 16);
        let palette_data_low = ppu.vram[(addr + 0 + y) as usize];
        let palette_data_high = ppu.vram[(addr + 8 + y) as usize];
        for x in 0..8 {
            let palette_num = (
                 ((palette_data_low  >> (7 - x)) & 1) |
                (((palette_data_high >> (7 - x)) & 1) << 1)
            ) as u8;
            let (r, g, b) = get_palette(ppu, palette_num, palette_offset);
            put_pixel(canvas, base_x + x, base_y + (y as i32), r, g, b);
        }
    }
}

#[inline(always)]
fn put_bg_tile(canvas: &mut Vec<u8>, ppu: &Ppu, base_x: i32, base_y: i32, chrnum: u8) {
    let base_addr = (((ppu.reg_controller >> 4) & 1) as u16) * 0x1000;
    // println!("put bg x:{} y:{} addr:{:04X}", base_x, base_y, base_addr);
    put_tile(canvas, ppu, base_x, base_y, base_addr, chrnum, 0x00);
}

pub fn draw_to_canvas(canvas: &mut Vec<u8>, ppu: &Ppu) {
    // println!("ppu controller:{:02X} mask:{:02X} status:{:02X}", ppu.reg_controller, ppu.reg_mask, ppu.reg_status);
    for y in 0..(240/8) {
        for x in 0..(256/8) {
            let bgaddr = ADDR_BG0 + y * 32 + x;
            let chrnum = ppu.vram[bgaddr as usize];
            put_bg_tile(canvas, ppu, (x * 8) as i32, (y * 8) as i32, chrnum);
        }
    }

    for i in 0..(256/4) {
        let base = i * 4;
        let y = ppu.oam[base + 0];
        let tile = ppu.oam[base + 1];
        let attr = ppu.oam[base + 2];
        let x = ppu.oam[base + 3];

        let base_addr = (((ppu.reg_controller >> 3) & 1) as u16) * 0x1000;
        // println!("put bg x:{} y:{} addr:{:04X}", base_x, base_y, base_addr);
        put_tile(canvas, ppu, x as i32, y as i32, base_addr, tile, 0x10);
    }
}