pub struct Opcode {
    pub code: u8,
    pub cycles: u8,
    pub addressing: u8,
    pub bytes: u8,
}

pub const OPCODE_BRK: u8 = 1;
pub const OPCODE_ORA: u8 = 2;
pub const OPCODE_STP: u8 = 3;
pub const OPCODE_SLO: u8 = 4;
pub const OPCODE_NOP: u8 = 5;
pub const OPCODE_ASL: u8 = 6;
pub const OPCODE_PHP: u8 = 7;
pub const OPCODE_ANC: u8 = 8;
pub const OPCODE_BPL: u8 = 9;
pub const OPCODE_CLC: u8 = 10;
pub const OPCODE_JSR: u8 = 11;
pub const OPCODE_BIT: u8 = 12;
pub const OPCODE_AND: u8 = 13;
pub const OPCODE_ROL: u8 = 14;
pub const OPCODE_RLA: u8 = 15;
pub const OPCODE_BMI: u8 = 16;
pub const OPCODE_SEC: u8 = 17;
pub const OPCODE_LDX: u8 = 18;
pub const OPCODE_LDA: u8 = 19;
pub const OPCODE_LDY: u8 = 20;
pub const OPCODE_LAX: u8 = 21;
pub const OPCODE_SAX: u8 = 22;
pub const OPCODE_ROR: u8 = 23;
pub const OPCODE_RRA: u8 = 24;
pub const OPCODE_RTI: u8 = 25;
pub const OPCODE_ROS: u8 = 26;
pub const OPCODE_CPX: u8 = 27;
pub const OPCODE_CPY: u8 = 28;
pub const OPCODE_CMP: u8 = 29;
pub const OPCODE_SBC: u8 = 30;
pub const OPCODE_EOR: u8 = 31;
pub const OPCODE_LSR: u8 = 32;
pub const OPCODE_PLP: u8 = 33;
pub const OPCODE_PHA: u8 = 34;
pub const OPCODE_SRE: u8 = 35;
pub const OPCODE_BVS: u8 = 36;
pub const OPCODE_DEC: u8 = 37;
pub const OPCODE_INC: u8 = 38;
pub const OPCODE_STX: u8 = 39;
pub const OPCODE_TYA: u8 = 40;
pub const OPCODE_SEI: u8 = 41;
pub const OPCODE_CLV: u8 = 42;
pub const OPCODE_ISC: u8 = 43;
pub const OPCODE_KIL: u8 = 44;
pub const OPCODE_ALR: u8 = 45;
pub const OPCODE_JMP: u8 = 46;
pub const OPCODE_BVC: u8 = 47;
pub const OPCODE_RTS: u8 = 48;
pub const OPCODE_ADC: u8 = 49;
pub const OPCODE_CLI: u8 = 50;
pub const OPCODE_ARR: u8 = 51;
pub const OPCODE_PLA: u8 = 52;
pub const OPCODE_STA: u8 = 53;
pub const OPCODE_STY: u8 = 54;
pub const OPCODE_DEY: u8 = 55;
pub const OPCODE_TAX: u8 = 56;
pub const OPCODE_TAY: u8 = 57;
pub const OPCODE_TXA: u8 = 58;
pub const OPCODE_XAA: u8 = 59;
pub const OPCODE_BCC: u8 = 60;
pub const OPCODE_AHX: u8 = 61;
pub const OPCODE_TXS: u8 = 62;
pub const OPCODE_TAS: u8 = 63;
pub const OPCODE_SHY: u8 = 64;
pub const OPCODE_SHX: u8 = 65;
pub const OPCODE_BCS: u8 = 66;
pub const OPCODE_TSX: u8 = 67;
pub const OPCODE_LAS: u8 = 68;
pub const OPCODE_DCP: u8 = 69;
pub const OPCODE_INY: u8 = 70;
pub const OPCODE_DEX: u8 = 71;
pub const OPCODE_AXS: u8 = 72;
pub const OPCODE_BNE: u8 = 73;
pub const OPCODE_CLD: u8 = 74;
pub const OPCODE_INX: u8 = 75;
pub const OPCODE_BEQ: u8 = 76;
pub const OPCODE_SED: u8 = 77;

pub const ADDRESSING_IMPLIED: u8 = 1;
pub const ADDRESSING_IMMEDIATE: u8 = 2;
pub const ADDRESSING_ZEROPAGE: u8 = 3;
pub const ADDRESSING_ZEROPAGE_X: u8 = 4;
pub const ADDRESSING_ZEROPAGE_Y: u8 = 5;
pub const ADDRESSING_ABSOLUTE: u8 = 6;
pub const ADDRESSING_ABSOLUTE_X: u8 = 7;
pub const ADDRESSING_ABSOLUTE_Y: u8 = 8;
pub const ADDRESSING_INDIRECT_X: u8 = 9;
pub const ADDRESSING_INDIRECT_Y: u8 = 10;
pub const ADDRESSING_ACCUMULATOR: u8 = 11;
pub const ADDRESSING_RELATIVE: u8 = 12;
pub const ADDRESSING_INDIRECT: u8 = 13;

pub const OPCODE_TABLE: [Opcode; 256] = [
    // 0x00
    Opcode { code: OPCODE_BRK, bytes:0, cycles:0, addressing: ADDRESSING_IMPLIED},
    Opcode { code: OPCODE_ORA, bytes:2, cycles:6, addressing: ADDRESSING_INDIRECT_X },
    Opcode { code: OPCODE_KIL, bytes:0, cycles:0,addressing: ADDRESSING_IMPLIED},
    Opcode { code: OPCODE_SLO, bytes:2, cycles:8, addressing: ADDRESSING_INDIRECT_X },
    Opcode { code: OPCODE_NOP, bytes:2, cycles:3, addressing: ADDRESSING_ZEROPAGE },
    Opcode { code: OPCODE_ORA, bytes:2, cycles:3, addressing: ADDRESSING_ZEROPAGE },
    Opcode { code: OPCODE_ASL, bytes:2, cycles:5, addressing: ADDRESSING_ZEROPAGE },
    Opcode { code: OPCODE_SLO, bytes:2, cycles:5, addressing: ADDRESSING_ZEROPAGE },
    Opcode { code: OPCODE_PHP, bytes:1, cycles:3, addressing: ADDRESSING_IMPLIED},
    Opcode { code: OPCODE_ORA, bytes:2, cycles:2, addressing: ADDRESSING_IMMEDIATE },
    Opcode { code: OPCODE_ASL, bytes:1, cycles:2, addressing: ADDRESSING_IMPLIED},
    Opcode { code: OPCODE_ANC, bytes:2, cycles:2, addressing: ADDRESSING_IMMEDIATE },
    Opcode { code: OPCODE_NOP, bytes:3, cycles:4, addressing: ADDRESSING_ABSOLUTE },
    Opcode { code: OPCODE_ORA, bytes:3, cycles:4, addressing: ADDRESSING_ABSOLUTE },
    Opcode { code: OPCODE_ASL, bytes:3, cycles:6, addressing: ADDRESSING_ABSOLUTE },
    Opcode { code: OPCODE_SLO, bytes:3, cycles:6, addressing: ADDRESSING_ABSOLUTE },
    // 0x10
    Opcode { code: OPCODE_BPL, bytes:2, cycles:3, addressing: ADDRESSING_RELATIVE },
    Opcode { code: OPCODE_ORA, bytes:2, cycles:5, addressing: ADDRESSING_INDIRECT_Y },
    Opcode { code: OPCODE_KIL, bytes:0, cycles:0,addressing: ADDRESSING_IMPLIED},
    Opcode { code: OPCODE_SLO, bytes:2, cycles:8, addressing: ADDRESSING_INDIRECT_Y },
    Opcode { code: OPCODE_NOP, bytes:2, cycles:4, addressing: ADDRESSING_ZEROPAGE_X },
    Opcode { code: OPCODE_ORA, bytes:2, cycles:4, addressing: ADDRESSING_ZEROPAGE_X },
    Opcode { code: OPCODE_ASL, bytes:2, cycles:6, addressing: ADDRESSING_ZEROPAGE_X },
    Opcode { code: OPCODE_SLO, bytes:2, cycles:6, addressing: ADDRESSING_ZEROPAGE_X },
    Opcode { code: OPCODE_CLC, bytes:1, cycles:2, addressing: ADDRESSING_IMPLIED},
    Opcode { code: OPCODE_ORA, bytes:3, cycles:4, addressing: ADDRESSING_ABSOLUTE_Y },
    Opcode { code: OPCODE_NOP, bytes:1, cycles:2, addressing: ADDRESSING_IMPLIED},
    Opcode { code: OPCODE_SLO, bytes:3, cycles:7, addressing: ADDRESSING_ABSOLUTE_Y },
    Opcode { code: OPCODE_NOP, bytes:3, cycles:4, addressing: ADDRESSING_ABSOLUTE_X },
    Opcode { code: OPCODE_ORA, bytes:3, cycles:4, addressing: ADDRESSING_ABSOLUTE_X },
    Opcode { code: OPCODE_ASL, bytes:3, cycles:7, addressing: ADDRESSING_ABSOLUTE_X },
    Opcode { code: OPCODE_SLO, bytes:3, cycles:7, addressing: ADDRESSING_ABSOLUTE_X },
    // 0x20
    Opcode { code: OPCODE_JSR, bytes:3, cycles:6, addressing: ADDRESSING_ABSOLUTE },
    Opcode { code: OPCODE_AND, bytes:2, cycles:6, addressing: ADDRESSING_INDIRECT_X },
    Opcode { code: OPCODE_KIL, bytes:0, cycles:0,addressing: ADDRESSING_IMPLIED},
    Opcode { code: OPCODE_RLA, bytes:2, cycles:8, addressing: ADDRESSING_INDIRECT_X },
    Opcode { code: OPCODE_BIT, bytes:2, cycles:3, addressing: ADDRESSING_ZEROPAGE },
    Opcode { code: OPCODE_AND, bytes:2, cycles:3, addressing: ADDRESSING_ZEROPAGE },
    Opcode { code: OPCODE_ROL, bytes:2, cycles:5, addressing: ADDRESSING_ZEROPAGE },
    Opcode { code: OPCODE_RLA, bytes:2, cycles:5, addressing: ADDRESSING_ZEROPAGE },
    Opcode { code: OPCODE_PLP, bytes:1, cycles:4, addressing: ADDRESSING_IMPLIED},
    Opcode { code: OPCODE_AND, bytes:2, cycles:2, addressing: ADDRESSING_IMMEDIATE },
    Opcode { code: OPCODE_ROL, bytes:1, cycles:2, addressing: ADDRESSING_IMPLIED},
    Opcode { code: OPCODE_ANC, bytes:2, cycles:2, addressing: ADDRESSING_IMMEDIATE },
    Opcode { code: OPCODE_BIT, bytes:3, cycles:4, addressing: ADDRESSING_ABSOLUTE },
    Opcode { code: OPCODE_AND, bytes:3, cycles:4, addressing: ADDRESSING_ABSOLUTE },
    Opcode { code: OPCODE_ROL, bytes:3, cycles:6, addressing: ADDRESSING_ABSOLUTE },
    Opcode { code: OPCODE_RLA, bytes:3, cycles:6, addressing: ADDRESSING_ABSOLUTE },
    // 0x30
    Opcode { code: OPCODE_BMI, bytes:2, cycles:2, addressing: ADDRESSING_RELATIVE },
    Opcode { code: OPCODE_AND, bytes:2, cycles:5, addressing: ADDRESSING_INDIRECT_Y },
    Opcode { code: OPCODE_KIL, bytes:0, cycles:0,addressing: ADDRESSING_IMPLIED},
    Opcode { code: OPCODE_RLA, bytes:2, cycles:8, addressing: ADDRESSING_INDIRECT_Y },
    Opcode { code: OPCODE_NOP, bytes:2, cycles:4, addressing: ADDRESSING_ZEROPAGE_X },
    Opcode { code: OPCODE_AND, bytes:2, cycles:4, addressing: ADDRESSING_ZEROPAGE_X },
    Opcode { code: OPCODE_ROL, bytes:2, cycles:6, addressing: ADDRESSING_ZEROPAGE_X },
    Opcode { code: OPCODE_RLA, bytes:2, cycles:6, addressing: ADDRESSING_ZEROPAGE_X },
    Opcode { code: OPCODE_SEC, bytes:1, cycles:2, addressing: ADDRESSING_IMPLIED},
    Opcode { code: OPCODE_AND, bytes:3, cycles:4, addressing: ADDRESSING_ABSOLUTE_Y },
    Opcode { code: OPCODE_NOP, bytes:1, cycles:2, addressing: ADDRESSING_IMPLIED},
    Opcode { code: OPCODE_RLA, bytes:3, cycles:7, addressing: ADDRESSING_ABSOLUTE_Y },
    Opcode { code: OPCODE_NOP, bytes:3, cycles:4, addressing: ADDRESSING_ABSOLUTE_X },
    Opcode { code: OPCODE_AND, bytes:3, cycles:4, addressing: ADDRESSING_ABSOLUTE_X },
    Opcode { code: OPCODE_ROL, bytes:3, cycles:7, addressing: ADDRESSING_ABSOLUTE_X },
    Opcode { code: OPCODE_RLA, bytes:3, cycles:7, addressing: ADDRESSING_ABSOLUTE_X },
    // 0x40
    Opcode { code: OPCODE_RTI, bytes:1, cycles:6, addressing: ADDRESSING_IMPLIED},
    Opcode { code: OPCODE_EOR, bytes:2, cycles:6, addressing: ADDRESSING_INDIRECT_X },
    Opcode { code: OPCODE_KIL, bytes:0, cycles:0,addressing: ADDRESSING_IMPLIED},
    Opcode { code: OPCODE_SRE, bytes:2, cycles:8, addressing: ADDRESSING_INDIRECT_X },
    Opcode { code: OPCODE_NOP, bytes:2, cycles:3, addressing: ADDRESSING_ZEROPAGE },
    Opcode { code: OPCODE_EOR, bytes:2, cycles:3, addressing: ADDRESSING_ZEROPAGE },
    Opcode { code: OPCODE_LSR, bytes:2, cycles:5, addressing: ADDRESSING_ZEROPAGE },
    Opcode { code: OPCODE_SRE, bytes:2, cycles:5, addressing: ADDRESSING_ZEROPAGE },
    Opcode { code: OPCODE_PHA, bytes:1, cycles:3, addressing: ADDRESSING_IMPLIED},
    Opcode { code: OPCODE_EOR, bytes:2, cycles:2, addressing: ADDRESSING_IMMEDIATE },
    Opcode { code: OPCODE_LSR, bytes:1, cycles:2, addressing: ADDRESSING_IMPLIED},
    Opcode { code: OPCODE_ALR, bytes:2, cycles:2, addressing: ADDRESSING_IMMEDIATE },
    Opcode { code: OPCODE_JMP, bytes:3, cycles:3, addressing: ADDRESSING_ABSOLUTE },
    Opcode { code: OPCODE_EOR, bytes:3, cycles:4, addressing: ADDRESSING_ABSOLUTE },
    Opcode { code: OPCODE_LSR, bytes:3, cycles:6, addressing: ADDRESSING_ABSOLUTE },
    Opcode { code: OPCODE_SRE, bytes:3, cycles:6, addressing: ADDRESSING_ABSOLUTE },
    // 0x50
    Opcode { code: OPCODE_BVC, bytes:2, cycles:3, addressing: ADDRESSING_RELATIVE },
    Opcode { code: OPCODE_EOR, bytes:2, cycles:5, addressing: ADDRESSING_INDIRECT_Y },
    Opcode { code: OPCODE_KIL, bytes:0, cycles:0,addressing: ADDRESSING_IMPLIED},
    Opcode { code: OPCODE_SRE, bytes:2, cycles:8, addressing: ADDRESSING_INDIRECT_Y },
    Opcode { code: OPCODE_NOP, bytes:2, cycles:4, addressing: ADDRESSING_ZEROPAGE_X },
    Opcode { code: OPCODE_EOR, bytes:2, cycles:4, addressing: ADDRESSING_ZEROPAGE_X },
    Opcode { code: OPCODE_LSR, bytes:2, cycles:6, addressing: ADDRESSING_ZEROPAGE_X },
    Opcode { code: OPCODE_SRE, bytes:2, cycles:6, addressing: ADDRESSING_ZEROPAGE_X },
    Opcode { code: OPCODE_CLI, bytes:1, cycles:2, addressing: ADDRESSING_IMPLIED},
    Opcode { code: OPCODE_EOR, bytes:3, cycles:4, addressing: ADDRESSING_ABSOLUTE_Y },
    Opcode { code: OPCODE_NOP, bytes:1, cycles:2, addressing: ADDRESSING_IMPLIED},
    Opcode { code: OPCODE_SRE, bytes:3, cycles:7, addressing: ADDRESSING_ABSOLUTE_Y },
    Opcode { code: OPCODE_NOP, bytes:3, cycles:4, addressing: ADDRESSING_ABSOLUTE_X },
    Opcode { code: OPCODE_EOR, bytes:3, cycles:4, addressing: ADDRESSING_ABSOLUTE_X },
    Opcode { code: OPCODE_LSR, bytes:3, cycles:7, addressing: ADDRESSING_ABSOLUTE_X },
    Opcode { code: OPCODE_SRE, bytes:3, cycles:7, addressing: ADDRESSING_ABSOLUTE_X },
    // 0x60
    Opcode { code: OPCODE_RTS, bytes:1, cycles:6, addressing: ADDRESSING_IMPLIED},
    Opcode { code: OPCODE_ADC, bytes:2, cycles:6, addressing: ADDRESSING_INDIRECT_X },
    Opcode { code: OPCODE_KIL, bytes:0, cycles:0,addressing: ADDRESSING_IMPLIED},
    Opcode { code: OPCODE_RRA, bytes:2, cycles:8, addressing: ADDRESSING_INDIRECT_X },
    Opcode { code: OPCODE_NOP, bytes:2, cycles:3, addressing: ADDRESSING_ZEROPAGE },
    Opcode { code: OPCODE_ADC, bytes:2, cycles:3, addressing: ADDRESSING_ZEROPAGE },
    Opcode { code: OPCODE_ROR, bytes:2, cycles:5, addressing: ADDRESSING_ZEROPAGE },
    Opcode { code: OPCODE_RRA, bytes:2, cycles:5, addressing: ADDRESSING_ZEROPAGE },
    Opcode { code: OPCODE_PLA, bytes:1, cycles:4, addressing: ADDRESSING_IMPLIED},
    Opcode { code: OPCODE_ADC, bytes:2, cycles:2, addressing: ADDRESSING_IMMEDIATE },
    Opcode { code: OPCODE_ROR, bytes:1, cycles:2, addressing: ADDRESSING_IMPLIED},
    Opcode { code: OPCODE_ARR, bytes:2, cycles:2, addressing: ADDRESSING_IMMEDIATE },
    Opcode { code: OPCODE_JMP, bytes:3, cycles:5, addressing: ADDRESSING_INDIRECT },
    Opcode { code: OPCODE_ADC, bytes:3, cycles:4, addressing: ADDRESSING_ABSOLUTE },
    Opcode { code: OPCODE_ROR, bytes:3, cycles:6, addressing: ADDRESSING_ABSOLUTE },
    Opcode { code: OPCODE_RRA, bytes:3, cycles:6, addressing: ADDRESSING_ABSOLUTE },
    // 0x70
    Opcode { code: OPCODE_BVS, bytes:2, cycles:2, addressing: ADDRESSING_RELATIVE },
    Opcode { code: OPCODE_ADC, bytes:2, cycles:5, addressing: ADDRESSING_INDIRECT_Y },
    Opcode { code: OPCODE_KIL, bytes:0, cycles:0,addressing: ADDRESSING_IMPLIED},
    Opcode { code: OPCODE_RRA, bytes:2, cycles:8, addressing: ADDRESSING_INDIRECT_Y },
    Opcode { code: OPCODE_NOP, bytes:2, cycles:4, addressing: ADDRESSING_ZEROPAGE_X },
    Opcode { code: OPCODE_ADC, bytes:2, cycles:4, addressing: ADDRESSING_ZEROPAGE_X },
    Opcode { code: OPCODE_ROR, bytes:2, cycles:6, addressing: ADDRESSING_ZEROPAGE_X },
    Opcode { code: OPCODE_RRA, bytes:2, cycles:6, addressing: ADDRESSING_ZEROPAGE_X },
    Opcode { code: OPCODE_SEI, bytes:1, cycles:2, addressing: ADDRESSING_IMPLIED},
    Opcode { code: OPCODE_ADC, bytes:3, cycles:4, addressing: ADDRESSING_ABSOLUTE_Y },
    Opcode { code: OPCODE_NOP, bytes:1, cycles:2, addressing: ADDRESSING_IMPLIED},
    Opcode { code: OPCODE_RRA, bytes:3, cycles:7, addressing: ADDRESSING_ABSOLUTE_Y },
    Opcode { code: OPCODE_NOP, bytes:3, cycles:4, addressing: ADDRESSING_ABSOLUTE_X },
    Opcode { code: OPCODE_ADC, bytes:3, cycles:4, addressing: ADDRESSING_ABSOLUTE_X },
    Opcode { code: OPCODE_ROR, bytes:3, cycles:7, addressing: ADDRESSING_ABSOLUTE_X },
    Opcode { code: OPCODE_RRA, bytes:3, cycles:7, addressing: ADDRESSING_ABSOLUTE_X },
    // 0x80
    Opcode { code: OPCODE_NOP, bytes:2, cycles:2, addressing: ADDRESSING_IMMEDIATE },
    Opcode { code: OPCODE_STA, bytes:2, cycles:6, addressing: ADDRESSING_INDIRECT_X },
    Opcode { code: OPCODE_NOP, bytes:2, cycles:2, addressing: ADDRESSING_IMMEDIATE },
    Opcode { code: OPCODE_SAX, bytes:2, cycles:6, addressing: ADDRESSING_INDIRECT_X },
    Opcode { code: OPCODE_STY, bytes:2, cycles:3, addressing: ADDRESSING_ZEROPAGE },
    Opcode { code: OPCODE_STA, bytes:2, cycles:3, addressing: ADDRESSING_ZEROPAGE },
    Opcode { code: OPCODE_STX, bytes:2, cycles:3, addressing: ADDRESSING_ZEROPAGE },
    Opcode { code: OPCODE_SAX, bytes:2, cycles:3, addressing: ADDRESSING_ZEROPAGE },
    Opcode { code: OPCODE_DEY, bytes:1, cycles:2, addressing: ADDRESSING_IMPLIED},
    Opcode { code: OPCODE_NOP, bytes:2, cycles:2, addressing: ADDRESSING_IMMEDIATE },
    Opcode { code: OPCODE_TXA, bytes:1, cycles:2, addressing: ADDRESSING_IMPLIED},
    Opcode { code: OPCODE_XAA, bytes:2, cycles:2, addressing: ADDRESSING_IMMEDIATE },
    Opcode { code: OPCODE_STY, bytes:3, cycles:4, addressing: ADDRESSING_ABSOLUTE },
    Opcode { code: OPCODE_STA, bytes:3, cycles:4, addressing: ADDRESSING_ABSOLUTE },
    Opcode { code: OPCODE_STX, bytes:3, cycles:4, addressing: ADDRESSING_ABSOLUTE },
    Opcode { code: OPCODE_SAX, bytes:3, cycles:4, addressing: ADDRESSING_ABSOLUTE },
    // 0x90
    Opcode { code: OPCODE_BCC, bytes:2, cycles:3, addressing: ADDRESSING_RELATIVE },
    Opcode { code: OPCODE_STA, bytes:2, cycles:6, addressing: ADDRESSING_INDIRECT_Y },
    Opcode { code: OPCODE_KIL, bytes:0, cycles:0,addressing: ADDRESSING_IMPLIED},
    Opcode { code: OPCODE_AHX, bytes:2, cycles:6, addressing: ADDRESSING_INDIRECT_Y },
    Opcode { code: OPCODE_STY, bytes:2, cycles:4, addressing: ADDRESSING_ZEROPAGE_X },
    Opcode { code: OPCODE_STA, bytes:2, cycles:4, addressing: ADDRESSING_ZEROPAGE_X },
    Opcode { code: OPCODE_STX, bytes:2, cycles:4, addressing: ADDRESSING_ZEROPAGE_Y },
    Opcode { code: OPCODE_SAX, bytes:2, cycles:4, addressing: ADDRESSING_ZEROPAGE_Y },
    Opcode { code: OPCODE_TYA, bytes:1, cycles:2, addressing: ADDRESSING_IMPLIED},
    Opcode { code: OPCODE_STA, bytes:3, cycles:5, addressing: ADDRESSING_ABSOLUTE_Y },
    Opcode { code: OPCODE_TXS, bytes:1, cycles:2, addressing: ADDRESSING_IMPLIED},
    Opcode { code: OPCODE_TAS, bytes:0, cycles:5, addressing: ADDRESSING_ABSOLUTE_Y }, // unknown bytes
    Opcode { code: OPCODE_SHY, bytes:3, cycles:5, addressing: ADDRESSING_ABSOLUTE_X },
    Opcode { code: OPCODE_STA, bytes:3, cycles:5, addressing: ADDRESSING_ABSOLUTE_X },
    Opcode { code: OPCODE_SHX, bytes:3, cycles:5, addressing: ADDRESSING_ABSOLUTE_Y },
    Opcode { code: OPCODE_AHX, bytes:3, cycles:5, addressing: ADDRESSING_ABSOLUTE_Y },
    // 0xA0
    Opcode { code: OPCODE_LDY, bytes:2, cycles:2, addressing: ADDRESSING_IMMEDIATE },
    Opcode { code: OPCODE_LDA, bytes:2, cycles:6, addressing: ADDRESSING_INDIRECT_X },
    Opcode { code: OPCODE_LDX, bytes:2, cycles:2, addressing: ADDRESSING_IMMEDIATE },
    Opcode { code: OPCODE_LAX, bytes:2, cycles:6, addressing: ADDRESSING_INDIRECT_X },
    Opcode { code: OPCODE_LDY, bytes:2, cycles:3, addressing: ADDRESSING_ZEROPAGE },
    Opcode { code: OPCODE_LDA, bytes:2, cycles:3, addressing: ADDRESSING_ZEROPAGE },
    Opcode { code: OPCODE_LDX, bytes:2, cycles:3, addressing: ADDRESSING_ZEROPAGE },
    Opcode { code: OPCODE_LAX, bytes:2, cycles:3, addressing: ADDRESSING_ZEROPAGE },
    Opcode { code: OPCODE_TAY, bytes:1, cycles:2, addressing: ADDRESSING_IMPLIED},
    Opcode { code: OPCODE_LDA, bytes:2, cycles:2, addressing: ADDRESSING_IMMEDIATE },
    Opcode { code: OPCODE_TAX, bytes:1, cycles:2, addressing: ADDRESSING_IMPLIED},
    Opcode { code: OPCODE_LAX, bytes:2, cycles:2, addressing: ADDRESSING_IMMEDIATE },
    Opcode { code: OPCODE_LDY, bytes:3, cycles:4, addressing: ADDRESSING_ABSOLUTE },
    Opcode { code: OPCODE_LDA, bytes:3, cycles:4, addressing: ADDRESSING_ABSOLUTE },
    Opcode { code: OPCODE_LDX, bytes:3, cycles:4, addressing: ADDRESSING_ABSOLUTE },
    Opcode { code: OPCODE_LAX, bytes:3, cycles:4, addressing: ADDRESSING_ABSOLUTE },
    // 0xB0
    Opcode { code: OPCODE_BCS, bytes:2, cycles:2, addressing: ADDRESSING_RELATIVE },
    Opcode { code: OPCODE_LDA, bytes:2, cycles:5, addressing: ADDRESSING_INDIRECT_Y },
    Opcode { code: OPCODE_KIL, bytes:0, cycles:0,addressing: ADDRESSING_IMPLIED},
    Opcode { code: OPCODE_LAX, bytes:2, cycles:5, addressing: ADDRESSING_INDIRECT_Y },
    Opcode { code: OPCODE_LDY, bytes:2, cycles:4, addressing: ADDRESSING_ZEROPAGE_X },
    Opcode { code: OPCODE_LDA, bytes:2, cycles:4, addressing: ADDRESSING_ZEROPAGE_X },
    Opcode { code: OPCODE_LDX, bytes:2, cycles:4, addressing: ADDRESSING_ZEROPAGE_Y },
    Opcode { code: OPCODE_LAX, bytes:2, cycles:4, addressing: ADDRESSING_ZEROPAGE_Y },
    Opcode { code: OPCODE_CLV, bytes:1, cycles:2, addressing: ADDRESSING_IMPLIED},
    Opcode { code: OPCODE_LDA, bytes:3, cycles:4, addressing: ADDRESSING_ABSOLUTE_Y },
    Opcode { code: OPCODE_TSX, bytes:1, cycles:2, addressing: ADDRESSING_IMPLIED},
    Opcode { code: OPCODE_LAS, bytes:3, cycles:4, addressing: ADDRESSING_ABSOLUTE_Y },
    Opcode { code: OPCODE_LDY, bytes:3, cycles:4, addressing: ADDRESSING_ABSOLUTE_X },
    Opcode { code: OPCODE_LDA, bytes:3, cycles:4, addressing: ADDRESSING_ABSOLUTE_X },
    Opcode { code: OPCODE_LDX, bytes:3, cycles:4, addressing: ADDRESSING_ABSOLUTE_Y },
    Opcode { code: OPCODE_LAX, bytes:3, cycles:4, addressing: ADDRESSING_ABSOLUTE_Y },
    // 0xC0
    Opcode { code: OPCODE_CPY, bytes:2, cycles:2, addressing: ADDRESSING_IMMEDIATE },
    Opcode { code: OPCODE_CMP, bytes:2, cycles:6, addressing: ADDRESSING_INDIRECT_X },
    Opcode { code: OPCODE_NOP, bytes:2, cycles:2, addressing: ADDRESSING_IMMEDIATE },
    Opcode { code: OPCODE_DCP, bytes:2, cycles:8, addressing: ADDRESSING_INDIRECT_X },
    Opcode { code: OPCODE_CPY, bytes:2, cycles:3, addressing: ADDRESSING_ZEROPAGE },
    Opcode { code: OPCODE_CMP, bytes:2, cycles:3, addressing: ADDRESSING_ZEROPAGE },
    Opcode { code: OPCODE_DEC, bytes:2, cycles:5, addressing: ADDRESSING_ZEROPAGE },
    Opcode { code: OPCODE_DCP, bytes:2, cycles:5, addressing: ADDRESSING_ZEROPAGE },
    Opcode { code: OPCODE_INY, bytes:1, cycles:2, addressing: ADDRESSING_IMPLIED},
    Opcode { code: OPCODE_CMP, bytes:2, cycles:2, addressing: ADDRESSING_IMMEDIATE },
    Opcode { code: OPCODE_DEX, bytes:1, cycles:2, addressing: ADDRESSING_IMPLIED},
    Opcode { code: OPCODE_AXS, bytes:2, cycles:2, addressing: ADDRESSING_IMMEDIATE },
    Opcode { code: OPCODE_CPY, bytes:3, cycles:4, addressing: ADDRESSING_ABSOLUTE },
    Opcode { code: OPCODE_CMP, bytes:3, cycles:4, addressing: ADDRESSING_ABSOLUTE },
    Opcode { code: OPCODE_DEC, bytes:3, cycles:6, addressing: ADDRESSING_ABSOLUTE },
    Opcode { code: OPCODE_DCP, bytes:3, cycles:6, addressing: ADDRESSING_ABSOLUTE },
    // 0xD0
    Opcode { code: OPCODE_BNE, bytes:2, cycles:3, addressing: ADDRESSING_RELATIVE },
    Opcode { code: OPCODE_CMP, bytes:2, cycles:5, addressing: ADDRESSING_INDIRECT_Y },
    Opcode { code: OPCODE_KIL, bytes:0, cycles:0,addressing: ADDRESSING_IMPLIED},
    Opcode { code: OPCODE_DCP, bytes:2, cycles:8, addressing: ADDRESSING_INDIRECT_Y },
    Opcode { code: OPCODE_NOP, bytes:2, cycles:4, addressing: ADDRESSING_ZEROPAGE_X },
    Opcode { code: OPCODE_CMP, bytes:2, cycles:4, addressing: ADDRESSING_ZEROPAGE_X },
    Opcode { code: OPCODE_DEC, bytes:2, cycles:6, addressing: ADDRESSING_ZEROPAGE_X },
    Opcode { code: OPCODE_DCP, bytes:2, cycles:6, addressing: ADDRESSING_ZEROPAGE_X },
    Opcode { code: OPCODE_CLD, bytes:1, cycles:2, addressing: ADDRESSING_IMPLIED},
    Opcode { code: OPCODE_CMP, bytes:3, cycles:4, addressing: ADDRESSING_ABSOLUTE_Y },
    Opcode { code: OPCODE_NOP, bytes:1, cycles:2, addressing: ADDRESSING_IMPLIED},
    Opcode { code: OPCODE_DCP, bytes:3, cycles:7, addressing: ADDRESSING_ABSOLUTE_Y },
    Opcode { code: OPCODE_NOP, bytes:3, cycles:4, addressing: ADDRESSING_ABSOLUTE_X },
    Opcode { code: OPCODE_CMP, bytes:3, cycles:4, addressing: ADDRESSING_ABSOLUTE_X },
    Opcode { code: OPCODE_DEC, bytes:3, cycles:7, addressing: ADDRESSING_ABSOLUTE_X },
    Opcode { code: OPCODE_DCP, bytes:3, cycles:7, addressing: ADDRESSING_ABSOLUTE_X },
    // 0xE0
    Opcode { code: OPCODE_CPX, bytes:2, cycles:2, addressing: ADDRESSING_IMMEDIATE },
    Opcode { code: OPCODE_SBC, bytes:2, cycles:6, addressing: ADDRESSING_INDIRECT_X },
    Opcode { code: OPCODE_NOP, bytes:2, cycles:2, addressing: ADDRESSING_IMMEDIATE },
    Opcode { code: OPCODE_ISC, bytes:2, cycles:8, addressing: ADDRESSING_INDIRECT_X },
    Opcode { code: OPCODE_CPX, bytes:2, cycles:3, addressing: ADDRESSING_ZEROPAGE },
    Opcode { code: OPCODE_SBC, bytes:2, cycles:3, addressing: ADDRESSING_ZEROPAGE },
    Opcode { code: OPCODE_INC, bytes:2, cycles:5, addressing: ADDRESSING_ZEROPAGE },
    Opcode { code: OPCODE_ISC, bytes:2, cycles:5, addressing: ADDRESSING_ZEROPAGE },
    Opcode { code: OPCODE_INX, bytes:1, cycles:2, addressing: ADDRESSING_IMPLIED},
    Opcode { code: OPCODE_SBC, bytes:2, cycles:2, addressing: ADDRESSING_IMMEDIATE },
    Opcode { code: OPCODE_NOP, bytes:1, cycles:2, addressing: ADDRESSING_IMPLIED},
    Opcode { code: OPCODE_SBC, bytes:2, cycles:2, addressing: ADDRESSING_IMMEDIATE },
    Opcode { code: OPCODE_CPX, bytes:3, cycles:4, addressing: ADDRESSING_ABSOLUTE },
    Opcode { code: OPCODE_SBC, bytes:3, cycles:4, addressing: ADDRESSING_ABSOLUTE },
    Opcode { code: OPCODE_INC, bytes:3, cycles:6, addressing: ADDRESSING_ABSOLUTE },
    Opcode { code: OPCODE_ISC, bytes:3, cycles:6, addressing: ADDRESSING_ABSOLUTE },
    // 0xF0
    Opcode { code: OPCODE_BEQ, bytes:2, cycles:2, addressing: ADDRESSING_RELATIVE },
    Opcode { code: OPCODE_SBC, bytes:2, cycles:5, addressing: ADDRESSING_INDIRECT_Y },
    Opcode { code: OPCODE_KIL, bytes:0, cycles:0,addressing: ADDRESSING_IMPLIED},
    Opcode { code: OPCODE_ISC, bytes:2, cycles:8, addressing: ADDRESSING_INDIRECT_Y },
    Opcode { code: OPCODE_NOP, bytes:2, cycles:4, addressing: ADDRESSING_ZEROPAGE_X },
    Opcode { code: OPCODE_SBC, bytes:2, cycles:4, addressing: ADDRESSING_ZEROPAGE_X },
    Opcode { code: OPCODE_INC, bytes:2, cycles:6, addressing: ADDRESSING_ZEROPAGE_X },
    Opcode { code: OPCODE_ISC, bytes:2, cycles:6, addressing: ADDRESSING_ZEROPAGE_X },
    Opcode { code: OPCODE_SED, bytes:1, cycles:2, addressing: ADDRESSING_IMPLIED},
    Opcode { code: OPCODE_SBC, bytes:3, cycles:4, addressing: ADDRESSING_ABSOLUTE_Y },
    Opcode { code: OPCODE_NOP, bytes:1, cycles:2, addressing: ADDRESSING_IMPLIED},
    Opcode { code: OPCODE_ISC, bytes:3, cycles:7, addressing: ADDRESSING_ABSOLUTE_Y },
    Opcode { code: OPCODE_NOP, bytes:3, cycles:4, addressing: ADDRESSING_ABSOLUTE_X },
    Opcode { code: OPCODE_SBC, bytes:3, cycles:4, addressing: ADDRESSING_ABSOLUTE_X },
    Opcode { code: OPCODE_INC, bytes:3, cycles:7, addressing: ADDRESSING_ABSOLUTE_X },
    Opcode { code: OPCODE_ISC, bytes:3, cycles:7, addressing: ADDRESSING_ABSOLUTE_X },
];

pub const OPCODE_DEBUG_SYMBOL: [&str; 256] = [
    "00 BRK 7",
    "01 ORA izx 6",
    "02 *KIL",
    "03 *SLO izx 8",
    "04 *NOP zp 3",
    "05 ORA zp 3",
    "06 ASL zp 5",
    "07 *SLO zp 5",
    "08 PHP 3",
    "09 ORA imm 2",
    "0A ASL 2",
    "0B *ANC imm 2",
    "0C *NOP abs 4",
    "0D ORA abs 4",
    "0E ASL abs 6",
    "0F *SLO abs 6",
    "10 BPL rel 2*",
    "11 ORA izy 5*",
    "12 *KIL",
    "13 *SLO izy 8",
    "14 *NOP zpx 4",
    "15 ORA zpx 4",
    "16 ASL zpx 6",
    "17 *SLO zpx 6",
    "18 CLC 2",
    "19 ORA aby 4*",
    "1A *NOP 2",
    "1B *SLO aby 7",
    "1C *NOP abx 4*",
    "1D ORA abx 4*",
    "1E ASL abx 7",
    "1F *SLO abx 7",
    "20 JSR abs 6",
    "21 AND izx 6",
    "22 *KIL",
    "23 *RLA izx 8",
    "24 BIT zp 3",
    "25 AND zp 3",
    "26 ROL zp 5",
    "27 *RLA zp 5",
    "28 PLP 4",
    "29 AND imm 2",
    "2A ROL 2",
    "2B *ANC imm 2",
    "2C BIT abs 4",
    "2D AND abs 4",
    "2E ROL abs 6",
    "2F *RLA abs 6",
    "30 BMI rel 2*",
    "31 AND izy 5*",
    "32 *KIL",
    "33 *RLA izy 8",
    "34 *NOP zpx 4",
    "35 AND zpx 4",
    "36 ROL zpx 6",
    "37 *RLA zpx 6",
    "38 SEC 2",
    "39 AND aby 4*",
    "3A *NOP 2",
    "3B *RLA aby 7",
    "3C *NOP abx 4*",
    "3D AND abx 4*",
    "3E ROL abx 7",
    "3F *RLA abx 7",
    "40 RTI 6",
    "41 EOR izx 6",
    "42 *KIL",
    "43 *SRE izx 8",
    "44 *NOP zp 3",
    "45 EOR zp 3",
    "46 LSR zp 5",
    "47 *SRE zp 5",
    "48 PHA 3",
    "49 EOR imm 2",
    "4A LSR 2",
    "4B *ALR imm 2",
    "4C JMP abs 3",
    "4D EOR abs 4",
    "4E LSR abs 6",
    "4F *SRE abs 6",
    "50 BVC rel 2*",
    "51 EOR izy 5*",
    "52 *KIL",
    "53 *SRE izy 8",
    "54 *NOP zpx 4",
    "55 EOR zpx 4",
    "56 LSR zpx 6",
    "57 *SRE zpx 6",
    "58 CLI 2",
    "59 EOR aby 4*",
    "5A *NOP 2",
    "5B *SRE aby 7",
    "5C *NOP abx 4*",
    "5D EOR abx 4*",
    "5E LSR abx 7",
    "5F *SRE abx 7",
    "60 RTS 6",
    "61 ADC izx 6",
    "62 *KIL",
    "63 *RRA izx 8",
    "64 *NOP zp 3",
    "65 ADC zp 3",
    "66 ROR zp 5",
    "67 *RRA zp 5",
    "68 PLA 4",
    "69 ADC imm 2",
    "6A ROR 2",
    "6B *ARR imm 2",
    "6C JMP ind 5",
    "6D ADC abs 4",
    "6E ROR abs 6",
    "6F *RRA abs 6",
    "70 BVS rel 2*",
    "71 ADC izy 5*",
    "72 *KIL",
    "73 *RRA izy 8",
    "74 *NOP zpx 4",
    "75 ADC zpx 4",
    "76 ROR zpx 6",
    "77 *RRA zpx 6",
    "78 SEI 2",
    "79 ADC aby 4*",
    "7A *NOP 2",
    "7B *RRA aby 7",
    "7C *NOP abx 4*",
    "7D ADC abx 4*",
    "7E ROR abx 7",
    "7F *RRA abx 7",
    "80 *NOP imm 2",
    "81 STA izx 6",
    "82 *NOP imm 2",
    "83 *SAX izx 6",
    "84 STY zp 3",
    "85 STA zp 3",
    "86 STX zp 3",
    "87 *SAX zp 3",
    "88 DEY 2",
    "89 *NOP imm 2",
    "8A TXA 2",
    "8B *XAA imm 2",
    "8C STY abs 4",
    "8D STA abs 4",
    "8E STX abs 4",
    "8F *SAX abs 4",
    "90 BCC rel 2*",
    "91 STA izy 6",
    "92 *KIL",
    "93 *AHX izy 6",
    "94 STY zpx 4",
    "95 STA zpx 4",
    "96 STX zpy 4",
    "97 *SAX zpy 4",
    "98 TYA 2",
    "99 STA aby 5",
    "9A TXS 2",
    "9B *TAS aby 5",
    "9C *SHY abx 5",
    "9D STA abx 5",
    "9E *SHX aby 5",
    "9F *AHX aby 5",
    "A0 LDY imm 2",
    "A1 LDA izx 6",
    "A2 LDX imm 2",
    "A3 *LAX izx 6",
    "A4 LDY zp 3",
    "A5 LDA zp 3",
    "A6 LDX zp 3",
    "A7 *LAX zp 3",
    "A8 TAY 2",
    "A9 LDA imm 2",
    "AA TAX 2",
    "AB *LAX imm 2",
    "AC LDY abs 4",
    "AD LDA abs 4",
    "AE LDX abs 4",
    "AF *LAX abs 4",
    "B0 BCS rel 2*",
    "B1 LDA izy 5*",
    "B2 *KIL",
    "B3 *LAX izy 5*",
    "B4 LDY zpx 4",
    "B5 LDA zpx 4",
    "B6 LDX zpy 4",
    "B7 *LAX zpy 4",
    "B8 CLV 2",
    "B9 LDA aby 4*",
    "BA TSX 2",
    "BB *LAS aby 4*",
    "BC LDY abx 4*",
    "BD LDA abx 4*",
    "BE LDX aby 4*",
    "BF *LAX aby 4*",
    "C0 CPY imm 2",
    "C1 CMP izx 6",
    "C2 *NOP imm 2",
    "C3 *DCP izx 8",
    "C4 CPY zp 3",
    "C5 CMP zp 3",
    "C6 DEC zp 5",
    "C7 *DCP zp 5",
    "C8 INY 2",
    "C9 CMP imm 2",
    "CA DEX 2",
    "CB *AXS imm 2",
    "CC CPY abs 4",
    "CD CMP abs 4",
    "CE DEC abs 6",
    "CF *DCP abs 6",
    "D0 BNE rel 2*",
    "D1 CMP izy 5*",
    "D2 *KIL",
    "D3 *DCP izy 8",
    "D4 *NOP zpx 4",
    "D5 CMP zpx 4",
    "D6 DEC zpx 6",
    "D7 *DCP zpx 6",
    "D8 CLD 2",
    "D9 CMP aby 4*",
    "DA *NOP 2",
    "DB *DCP aby 7",
    "DC *NOP abx 4*",
    "DD CMP abx 4*",
    "DE DEC abx 7",
    "DF *DCP abx 7",
    "E0 CPX imm 2",
    "E1 SBC izx 6",
    "E2 *NOP imm 2",
    "E3 *ISC izx 8",
    "E4 CPX zp 3",
    "E5 SBC zp 3",
    "E6 INC zp 5",
    "E7 *ISC zp 5",
    "E8 INX 2",
    "E9 SBC imm 2",
    "EA NOP 2",
    "EB *SBC imm 2",
    "EC CPX abs 4",
    "ED SBC abs 4",
    "EE INC abs 6",
    "EF *ISC abs 6",
    "F0 BEQ rel 2*",
    "F1 SBC izy 5*",
    "F2 *KIL",
    "F3 *ISC izy 8",
    "F4 *NOP zpx 4",
    "F5 SBC zpx 4",
    "F6 INC zpx 6",
    "F7 *ISC zpx 6",
    "F8 SED 2",
    "F9 SBC aby 4*",
    "FA *NOP 2",
    "FB *ISC aby 7",
    "FC *NOP abx 4*",
    "FD SBC abx 4*",
    "FE INC abx 7",
    "FF *ISC abx",

];

pub fn debug_opcode(code: u8) {
    println!("{}", OPCODE_DEBUG_SYMBOL[code as usize]);
}