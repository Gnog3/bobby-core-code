#[derive(Debug, PartialEq, Clone, Copy)]
pub struct RType {
    pub funct7: u32,
    pub rs2: usize,
    pub rs1: usize,
    pub funct3: u32,
    pub rd: usize,
}

impl From<u32> for RType {
    fn from(insn: u32) -> Self {
        Self {
            funct7: (insn >> 25) & 0x7f,
            rs2: ((insn >> 20) & 0x1f) as usize,
            rs1: ((insn >> 15) & 0x1f) as usize,
            funct3: (insn >> 12) & 0x7,
            rd: ((insn >> 7) & 0x1f) as usize,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct IType {
    pub imm: i32,
    pub rs1: usize,
    pub funct3: u32,
    pub rd: usize,
}

impl From<u32> for IType {
    fn from(insn: u32) -> Self {
        let uimm: i32 = ((insn >> 20) & 0x7ff) as i32;

        let imm: i32 = if (insn & 0x8000_0000) != 0 {
            uimm - (1 << 11)
        } else {
            uimm
        };

        Self {
            imm,
            rs1: ((insn >> 15) & 0x1f) as usize,
            funct3: (insn >> 12) & 0x7,
            rd: ((insn >> 7) & 0x1f) as usize,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct SType {
    pub imm: i32,
    pub rs2: usize,
    pub rs1: usize,
    pub funct3: u32,
}

impl From<u32> for SType {
    fn from(insn: u32) -> Self {
        let uimm: i32 = (((insn >> 20) & 0x7e0) | ((insn >> 7) & 0x1f)) as i32;

        let imm: i32 = if (insn & 0x8000_0000) != 0 {
            uimm - (1 << 11)
        } else {
            uimm
        };

        Self {
            imm,
            rs2: ((insn >> 20) & 0x1f) as usize,
            rs1: ((insn >> 15) & 0x1f) as usize,
            funct3: (insn >> 12) & 0x7,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct BType {
    pub imm: i32,
    pub rs2: usize,
    pub rs1: usize,
    pub funct3: u32,
}

impl From<u32> for BType {
    fn from(insn: u32) -> Self {
        let uimm: i32 =
            (((insn >> 20) & 0x7e0) | ((insn >> 7) & 0x1e) | ((insn & 0x80) << 4)) as i32;

        let imm: i32 = if (insn & 0x8000_0000) != 0 {
            uimm - (1 << 12)
        } else {
            uimm
        };

        Self {
            imm,
            rs2: ((insn >> 20) & 0x1f) as usize,
            rs1: ((insn >> 15) & 0x1f) as usize,
            funct3: (insn >> 12) & 0x7,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct UType {
    pub imm: u32,
    pub rd: usize,
}

impl From<u32> for UType {
    fn from(insn: u32) -> Self {
        UType {
            imm: (insn & 0xffff_f000) as u32,
            rd: ((insn >> 7) & 0x1f) as usize,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct JType {
    pub imm: i32,
    pub rd: usize,
}

impl From<u32> for JType {
    fn from(insn: u32) -> Self {
        let uimm: i32 =
            ((insn & 0xff000) | ((insn & 0x100000) >> 9) | ((insn >> 20) & 0x7fe)) as i32;

        let imm: i32 = if (insn & 0x8000_0000) != 0 {
            uimm - (1 << 20)
        } else {
            uimm
        };

        JType {
            imm,
            rd: ((insn >> 7) & 0x1f) as usize,
        }
    }
}
