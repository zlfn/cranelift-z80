//! Register definitions for Z80.
//!
//! Z80 has the following registers:
//! - 8-bit general-purpose registers: A, B, C, D, E, H, L
//! - 16-bit register pairs: BC, DE, HL, AF
//! - 16-bit index registers: IX, IY (controlled by has_index_registers flag)
//! - 16-bit special registers: SP (Stack Pointer), PC (Program Counter)
//! - Shadow registers: AF', BC', DE', HL' (controlled by has_shadow_registers flag)

use crate::Reg;
use regalloc2::{VReg, PReg, RegClass};

/// Z80 8-bit General Purpose Registers
///
/// These are the ACTUAL Z80 hardware encodings used in machine code.
/// In most Z80 instructions, 8-bit registers are encoded as 3 bits:
///   - 000 (0) = B
///   - 001 (1) = C
///   - 010 (2) = D
///   - 011 (3) = E
///   - 100 (4) = H
///   - 101 (5) = L
///   - 110 (6) = (HL) - memory indirect, not a register
///   - 111 (7) = A - accumulator
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpr8 {
    B = 0b000,
    C = 0b001,
    D = 0b010,
    E = 0b011,
    H = 0b100,
    L = 0b101,
    A = 0b111,
}

#[inline]
#[allow(dead_code)]
pub(crate) fn b() -> Reg {
    gpr(Gpr8::B)
}

#[inline]
#[allow(dead_code)]
pub(crate) fn c() -> Reg {
    gpr(Gpr8::C)
}

#[inline]
#[allow(dead_code)]
pub(crate) fn d() -> Reg {
    gpr(Gpr8::D)
}

#[inline]
#[allow(dead_code)]
pub(crate) fn e() -> Reg {
    gpr(Gpr8::E)
}

#[inline]
#[allow(dead_code)]
pub(crate) fn h() -> Reg {
    gpr(Gpr8::H)
}

#[inline]
#[allow(dead_code)]
pub(crate) fn l() -> Reg {
    gpr(Gpr8::L)
}

#[inline]
#[allow(dead_code)]
pub(crate) fn a() -> Reg {
    gpr(Gpr8::A)
}

/// Z80 16-bit General Purpose Register Pairs
///
/// These are the ACTUAL Z80 hardware encodings used in machine code.
/// In most Z80 instructions, 16-bit register pairs are encoded as 2 bits:
///   00 (0) = BC
///   01 (1) = DE
///   10 (2) = HL
///   11 (3) = AF
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpp16 {
    BC = 0b00,
    DE = 0b01,
}

impl Gpp16 {
    fn to_gpr8_high_low(self) -> (Gpr8, Gpr8) {
        match self {
            Gpp16::BC => (Gpr8::B, Gpr8::C),
            Gpp16::DE => (Gpr8::D, Gpr8::E),
        }
    }
}

//=============================================================================
// Register Constructor Helpers

const fn gpr(r: Gpr8) -> Reg {
    let preg = preg(r);
    Reg::from_virtual_reg(VReg::new(preg.index(), RegClass::Int))
}

#[inline]
pub const fn preg(r: Gpr8) -> PReg {
    PReg::new(r as usize, RegClass::Int)
}
