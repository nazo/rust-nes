use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::str;
use std::error::Error;

#[derive(Debug)]
pub struct NesHeader {
    pub size_of_prg_rom: u32,
    pub size_of_chr_rom: u32,
    pub flag6: u8,
    pub flag7: u8,
    pub flag8: u8,
    pub flag9: u8,
    pub flag10: u8,
}

pub struct CharacterRom {
    pub data: Vec<u8>,
}

pub struct ProgramRom {
    pub data: Vec<u8>,
}

pub struct NesRom {
    pub header: NesHeader,
    pub program_rom: ProgramRom,
    pub character_rom: CharacterRom,
}

const NES_HEADER_SIZE: usize = 0x10;

fn load_program_rom(buffer: &[u8], header: &NesHeader) -> Result<ProgramRom, std::io::Error> {
    let start: usize = NES_HEADER_SIZE;
    let end = start + header.size_of_prg_rom as usize; 
    return Ok(ProgramRom {
        data: (&buffer[start..end]).to_vec(),
    })
}

fn load_character_rom(buffer: &[u8], header: &NesHeader) -> Result<CharacterRom, std::io::Error> {
    let start: usize = NES_HEADER_SIZE + header.size_of_prg_rom as usize;
    let end = start + header.size_of_chr_rom as usize; 
    return Ok(CharacterRom {
        data: (&buffer[start..end]).to_vec(),
    })
}

fn load_nes_header(buffer: &[u8]) -> Result<NesHeader, std::io::Error> {
    let file_header = &buffer[0..4];
    if file_header != [0x4E, 0x45, 0x53, 0x1A] {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Invalid file header. {:?}", file_header)
            ));
    }

    let header = &buffer[4..16];
    return Ok(NesHeader {
        size_of_prg_rom: (header[0] as u32) * 0x4000,
        size_of_chr_rom: (header[1] as u32) * 0x2000,
        flag6: header[2],
        flag7: header[3],
        flag8: header[4],
        flag9: header[5],
        flag10: header[6],
    })
}

pub fn load_nes_data(buffer: &[u8]) -> Result<NesRom, std::io::Error> {
    let nes_header: NesHeader;
    match load_nes_header(&buffer) {
        Err(why) => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("couldn't read header: {}", why.description())
                ));
        },
        Ok(header) => { nes_header = header; }
    }

    let program_rom: ProgramRom;
    match load_program_rom(&buffer, &nes_header) {
        Err(why) => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("couldn't read program rom: {}", why.description())
                ));
        },
        Ok(rom) => { program_rom = rom; },
    }

    let character_rom: CharacterRom;
    match load_character_rom(&buffer, &nes_header) {
        Err(why) => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("couldn't read character rom: {}", why.description())
                ));
        },
        Ok(rom) => { character_rom = rom; },
    }

    return Ok(NesRom {
        header: nes_header,
        program_rom: program_rom,
        character_rom: character_rom,
    })
}

pub fn load_file(filename: &str) -> Vec<u8> {
    // Create a path to the desired file
    let path = Path::new(filename);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    let filesize: u64;
    match file.metadata() {
        Err(why) => panic!("couldn't resolve metadata {}: {}", display, why.description()),
        Ok(metadata) => {
            filesize = metadata.len();
        },
    }

    let mut buffer = vec![0; filesize as usize];
    match file.read(&mut buffer) {
        Err(why) => panic!("couldn't read file {}: {}", display, why.description()),
        Ok(_) => {}
    }
    return buffer;
}

pub fn load_nes(filename: &str) -> NesRom {
    let rom_buffer = load_file(filename);
    match load_nes_data(&rom_buffer) {
        Err(why) => panic!("couldn't read rom: {}", why.description()),
        Ok(nes_rom) => {
            return nes_rom;
        }
    }
}