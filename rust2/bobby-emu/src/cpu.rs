use anyhow::{Result, bail};
use fps_counter::FPSCounter;

use crate::{
    csrs::Csrs,
    instruction_formats::{BType, IType, JType, RType, SType, UType},
    memory::{MemAccessSize, Memory},
};

pub struct Cpu {
    pub registers: [u32; 32],
    pub pc: u32,
    pub csrs: Csrs,
    pub mem: Memory,
    pub insn_count: u32,
    pub fps_counter: FPSCounter,
    pub fps: usize,
}

impl Cpu {
    pub fn new(csrs: Csrs) -> Self {
        Cpu {
            registers: [0; 32],
            pc: 0,
            csrs,
            mem: Memory::new(16 * 1024 * 1024),
            insn_count: 0,
            fps_counter: FPSCounter::new(),
            fps: 0,
        }
    }

    pub fn flash(&mut self, data: &[u8]) {
        self.mem.flash(data);
    }

    fn write_register(&mut self, reg_index: usize, data: u32) {
        if reg_index == 0 {
            return;
        }

        self.registers[reg_index] = data;
    }

    pub fn read_register(&self, reg_index: usize) -> u32 {
        if reg_index == 0 {
            0
        } else {
            self.registers[reg_index]
        }
    }

    fn write_csr(&mut self, csr_addr: u32, data: u32) -> Result<()> {
        let csr = self.csrs.get_csr(csr_addr)?;
        csr.write(csr_addr, &mut self.mem.vec, data)
    }

    fn read_csr(&mut self, csr_addr: u32) -> Result<u32> {
        let csr = self.csrs.get_csr(csr_addr)?;
        csr.read(csr_addr, &mut self.mem.vec)
    }

    pub fn tick(&mut self) -> Result<()> {
        let insn = self.mem.read(self.pc, MemAccessSize::Word);
        let opcode = insn & 0x7F;
        match opcode {
            0b0110111 => self.lui(insn.into()),
            0b0010111 => self.auipc(insn.into()),
            0b1101111 => self.jal(insn.into()),
            0b1100111 => self.jalr(insn.into())?,
            0b1100011 => self.branch(insn.into())?,
            0b0000011 => self.load(insn.into())?,
            0b0100011 => self.store(insn.into())?,
            0b0010011 => self.alu_imm(insn.into())?,
            0b0110011 => self.alu(insn.into())?,
            0b1110011 => self.csr(insn.into())?,
            _ => bail!("invalid opcode: {}", opcode),
        }
        self.insn_count += 1;
        self.fps = self.fps_counter.tick();
        Ok(())
    }

    fn lui(&mut self, insn: UType) {
        self.write_register(insn.rd, insn.imm);
        self.pc += 4;
    }

    fn auipc(&mut self, insn: UType) {
        self.write_register(insn.rd, self.pc.wrapping_add(insn.imm));
        self.pc += 4;
    }

    fn jal(&mut self, insn: JType) {
        self.write_register(insn.rd, self.pc + 4);

        self.pc = self.pc.wrapping_add(insn.imm as u32);
    }

    fn jalr(&mut self, insn: IType) -> Result<()> {
        if insn.funct3 != 0 {
            bail!("invalid funct3 in jalr: {}", insn.funct3);
        }
        let target = self.read_register(insn.rs1).wrapping_add(insn.imm as u32) & !1;
        self.write_register(insn.rd, self.pc + 4);
        self.pc = target;
        Ok(())
    }

    fn branch(&mut self, insn: BType) -> Result<()> {
        let l = self.read_register(insn.rs1);
        let r = self.read_register(insn.rs2);
        let do_branch = match insn.funct3 {
            0b000 => l == r,
            0b001 => l != r,
            0b100 => (l as i32) < (r as i32),
            0b101 => (l as i32) >= (r as i32),
            0b110 => l < r,
            0b111 => l >= r,
            _ => bail!("invalid funct3 in branch: {}", insn.funct3),
        };
        if do_branch {
            self.pc = self.pc.wrapping_add(insn.imm as u32);
        } else {
            self.pc += 4;
        }
        Ok(())
    }

    fn load(&mut self, insn: IType) -> Result<()> {
        let size = match insn.funct3 {
            0b000 => MemAccessSize::Byte,
            0b001 => MemAccessSize::HalfWord,
            0b010 => MemAccessSize::Word,
            0b100 => MemAccessSize::Byte,
            0b101 => MemAccessSize::HalfWord,
            _ => bail!("invalid funct3 in load: {}", insn.funct3),
        };
        let addr = self.read_register(insn.rs1).wrapping_add(insn.imm as u32);
        let data = self.mem.read(addr, size);
        let data = match insn.funct3 {
            0b000 => data as i8 as i32 as u32,
            0b001 => data as i16 as i32 as u32,
            _ => data,
        };
        self.write_register(insn.rd, data);
        self.pc += 4;
        Ok(())
    }

    fn store(&mut self, insn: SType) -> Result<()> {
        let addr = self.read_register(insn.rs1).wrapping_add(insn.imm as u32);
        let data = self.read_register(insn.rs2);
        let size = match insn.funct3 {
            0b000 => MemAccessSize::Byte,
            0b001 => MemAccessSize::HalfWord,
            0b010 => MemAccessSize::Word,
            _ => bail!("invalid funct3 in store: {}", insn.funct3),
        };
        self.mem.write(addr, size, data);
        self.pc += 4;
        Ok(())
    }

    fn alu_imm(&mut self, insn: IType) -> Result<()> {
        let data = self.read_register(insn.rs1);
        let result = match insn.funct3 {
            0b000 => data.wrapping_add(insn.imm as u32),
            0b010 => ((data as i32) < insn.imm) as u32,
            0b011 => (data < insn.imm as u32) as u32,
            0b100 => data ^ insn.imm as u32,
            0b110 => data | insn.imm as u32,
            0b111 => data & insn.imm as u32,
            0b001 => {
                let (shamt, flag) = shamt(insn)?;
                if flag {
                    bail!("flag mustn't be set")
                }
                data << shamt
            }
            0b101 => {
                let (shamt, flag) = shamt(insn)?;
                if !flag {
                    data >> shamt
                } else {
                    (data as i32 >> shamt as i32) as u32
                }
            }
            _ => unreachable!(),
        };
        self.write_register(insn.rd, result);
        self.pc += 4;
        Ok(())
    }

    fn alu(&mut self, insn: RType) -> Result<()> {
        let l = self.read_register(insn.rs1);
        let r = self.read_register(insn.rs2);
        let result = match (insn.funct3, insn.funct7) {
            (0b000, 0) => l.wrapping_add(r),
            (0b000, 0b100000) => l.wrapping_sub(r),
            (0b001, 0) => l << (r & 0b11111),
            (0b010, 0) => ((l as i32) < (r as i32)) as u32,
            (0b011, 0) => (l < r) as u32,
            (0b100, 0) => l ^ r,
            (0b101, 0) => l >> (r & 0b11111),
            (0b101, 0b100000) => ((l as i32) >> (r as i32 & 0b11111)) as u32,
            (0b110, 0) => l | r,
            (0b111, 0) => l & r,
            _ => bail!("invalid funct in alu: {} {}", insn.funct3, insn.funct7),
        };
        self.write_register(insn.rd, result);
        self.pc += 4;
        Ok(())
    }

    fn csr(&mut self, insn: IType) -> Result<()> {
        let csr = insn.imm as u32 & 0xFFF;
        if insn.funct3 != 0b001 {
            bail!("invalid funct in csr: {}", insn.funct3);
        }
        let to_write = self.read_register(insn.rs1);
        let data = if insn.rd != 0 { self.read_csr(csr)? } else { 0 };
        self.write_csr(csr, to_write)?;
        self.write_register(insn.rd, data);
        self.pc += 4;
        Ok(())
    }
}

fn shamt(insn: IType) -> Result<(u32, bool)> {
    let shamt = insn.imm as u32 & 0b11111;
    let flag = match insn.imm as u32 >> 5 {
        0 => false,
        0b100000 => true,
        _ => bail!("invalid flag: {}", insn.imm as u32 >> 5),
    };
    Ok((shamt, flag))
}
