//! This module defines Z80-specific machine instruction types.

use crate::binemit::{Addend, CodeOffset, Reloc};
use crate::ir::{ExternalName, TrapCode, Type, types};
use crate::isa::z80::settings as z80_settings;
use crate::isa::FunctionAlignment;
use crate::{CodegenError, CodegenResult, settings};
use crate::machinst::*;
use alloc::boxed::Box;
use core::slice;
use smallvec::{SmallVec, smallvec};
use std::fmt::{self, Write};
use std::string::{String, ToString};

pub mod regs;
mod emit;
mod emit_state;

pub use emit_state::EmitState;
use regs::*;

//=============================================================================
// Instructions (top level): definition

// `Inst` is defined inside ISLE as `MInst`. We publicly re-export it here.
pub use super::lower::isle::generated_code::MInst as Inst;

#[test]
#[cfg(target_pointer_width = "64")]
fn inst_size_test() {
    // This test will help with unintentionally growing the size
    // of the Inst enum.
    // TODO: Adjust this size once we define the actual instruction variants
    // assert_eq!(XX, std::mem::size_of::<Inst>());
}

impl Inst {
    /// Check if the instruction can be emitted given the current target
    /// architecture.
    fn is_available(&self, _emit_info: &EmitInfo) -> bool {
        // For now, all instructions are available on Z80
        true
    }
}

//=============================================================================
// Instruction constructors

impl Inst {
    /// Create a NOP instruction
    pub(crate) fn nop() -> Self {
        todo!("Implement NOP instruction constructor")
    }

    /// Create a load instruction
    pub(crate) fn load(
        _ty: Type,
        _from_addr: impl Into<Amode>,
        _to_reg: Writable<Reg>,
    ) -> Self {
        todo!("Implement load instruction constructor")
    }

    /// Create a store instruction
    pub(crate) fn store(
        _ty: Type,
        _from_reg: Reg,
        _to_addr: impl Into<Amode>,
    ) -> Self {
        todo!("Implement store instruction constructor")
    }

    /// Create an immediate load instruction
    pub fn imm(_imm_val: u8, _dst: Writable<Reg>) -> Inst {
        todo!("Implement immediate load constructor")
    }

    /// Create a call to a known function
    pub(crate) fn call_known(_target: ExternalName) -> Inst {
        todo!("Implement call_known constructor")
    }

    /// Create an unconditional jump
    pub(crate) fn jmp_known(_dst: MachLabel) -> Inst {
        todo!("Implement jmp_known constructor")
    }

    /// Create a conditional trap
    pub(crate) fn trap_if(_trap_code: TrapCode) -> Inst {
        todo!("Implement trap_if constructor")
    }
}

//=============================================================================
// Instructions: printing

impl PrettyPrint for Inst {
    fn pretty_print(&self, _size: u8) -> String {
        // TODO: Implement pretty printing for each instruction variant
        format!("z80_inst_{:?}", self)
    }
}

//=============================================================================
// Instructions: MachInst trait implementation

impl MachInst for Inst {
    type ABIMachineSpec = Z80ABIMachineSpec;

    fn get_operands(&mut self, _collector: &mut impl OperandVisitor) {
        // TODO: Implement operand collection for register allocation
        todo!("Implement get_operands")
    }

    fn is_move(&self) -> Option<(Writable<Reg>, Reg)> {
        // TODO: Detect register-to-register moves
        None
    }

    fn is_included_in_clobbers(&self) -> bool {
        // Most instructions should be included in clobbers
        true
    }

    fn is_trap(&self) -> bool {
        // TODO: Detect trap instructions
        false
    }

    fn is_args(&self) -> bool {
        // TODO: Detect Args pseudo-instruction
        false
    }

    fn is_term(&self) -> MachTerminator {
        // TODO: Detect terminator instructions (ret, jmp, etc.)
        MachTerminator::None
    }

    fn is_low_level_branch(&self) -> bool {
        false
    }

    fn is_mem_access(&self) -> bool {
        // TODO: Detect memory access instructions
        false
    }

    fn gen_move(dst_reg: Writable<Reg>, src_reg: Reg, _ty: Type) -> Inst {
        // TODO: Generate register-to-register move
        // For Z80, this would typically be LD dst, src
        let _ = (dst_reg, src_reg);
        todo!("Implement gen_move")
    }

    fn gen_nop(_preferred_size: usize) -> Inst {
        Inst::nop()
    }

    fn rc_for_type(ty: Type) -> CodegenResult<(&'static [RegClass], &'static [Type])> {
        match ty {
            types::I8 => Ok((&[RegClass::Int], &[types::I8])),
            types::I16 => Ok((&[RegClass::Int, RegClass::Int], &[types::I8, types::I8])),
            types::I32 => Ok((
                &[RegClass::Int, RegClass::Int, RegClass::Int, RegClass::Int],
                &[types::I8, types::I8, types::I8, types::I8]
            )),
            types::I64 => Ok((
                &[
                    RegClass::Int, RegClass::Int, RegClass::Int, RegClass::Int,
                    RegClass::Int, RegClass::Int, RegClass::Int, RegClass::Int,
                ],
                &[
                    types::I8, types::I8, types::I8, types::I8,
                    types::I8, types::I8, types::I8, types::I8,
                ]
            )),
            // Z80 doesn't have native floating-point support
            _ => Err(CodegenError::Unsupported(format!(
                "Unsupported type for Z80: {ty}"
            ))),
        }
    }

    fn canonical_type_for_rc(rc: RegClass) -> Type {
        match rc {
            RegClass::Int => types::I8,
            _ => panic!("Unexpected register class for Z80: {:?}", rc),
        }
    }

    fn gen_jump(label: MachLabel) -> Inst {
        Inst::jmp_known(label)
    }

    fn gen_imm_u64(value: u64, dst: Writable<Reg>) -> Option<Self> {
        // Z80 can only load 8-bit immediates directly
        if value <= 0xFF {
            Some(Inst::imm(value as u8, dst))
        } else {
            None
        }
    }

    fn gen_imm_f64(_value: f64, _tmp: Writable<Reg>, _dst: Writable<Reg>) -> SmallVec<[Self; 2]> {
        // Z80 doesn't support floating-point
        panic!("Z80 does not support floating-point operations")
    }

    fn gen_dummy_use(_reg: Reg) -> Self {
        todo!("Implement gen_dummy_use")
    }

    fn worst_case_size() -> CodeOffset {
        // Z80 instructions are at most 4 bytes (with displacement and immediate)
        4
    }

    fn ref_type_regclass(_: &settings::Flags) -> RegClass {
        RegClass::Int
    }

    fn is_safepoint(&self) -> bool {
        // TODO: Detect call instructions
        false
    }

    fn function_alignment() -> FunctionAlignment {
        FunctionAlignment {
            minimum: 1,
            preferred: 1,
        }
    }

    type LabelUse = LabelUse;

    const TRAP_OPCODE: &'static [u8] = &[
        // TODO: Define the trap opcode for Z80
        // For now, we use HALT (0x76)
        0x76
    ];
}

//=============================================================================
// EmitInfo: constant state used during emission

/// Constant state used during emissions of a sequence of instructions.
pub struct EmitInfo {
    pub(super) flags: settings::Flags,
    pub(super) isa_flags: z80_settings::Flags,
}

impl EmitInfo {
    /// Create a constant state for emission of instructions.
    pub fn new(flags: settings::Flags, isa_flags: z80_settings::Flags) -> Self {
        Self { flags, isa_flags }
    }
}

//=============================================================================
// MachInstEmit trait implementation

impl MachInstEmit for Inst {
    type State = EmitState;
    type Info = EmitInfo;

    fn emit(&self, _sink: &mut MachBuffer<Inst>, _info: &Self::Info, _state: &mut Self::State) {
        // TODO: Implement instruction emission
        todo!("Implement emit")
    }

    fn pretty_print_inst(&self, _: &mut Self::State) -> String {
        PrettyPrint::pretty_print(self, 0)
    }
}

//=============================================================================
// LabelUse: handling of label references

/// A label-use (internal relocation) in generated code.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LabelUse {
    /// An 8-bit relative jump offset
    /// Used by JR instructions (2-byte: opcode + offset)
    JumpRel8,

    /// A 16-bit absolute address
    /// Used by JP and CALL instructions (3-byte: opcode + addr_low + addr_high)
    JumpAbs16,
}

impl MachInstLabelUse for LabelUse {
    const ALIGN: CodeOffset = 1;

    fn max_pos_range(self) -> CodeOffset {
        match self {
            LabelUse::JumpRel8 => 127,      // +127 bytes
            LabelUse::JumpAbs16 => 0xFFFF,  // 64KB address space
        }
    }

    fn max_neg_range(self) -> CodeOffset {
        match self {
            LabelUse::JumpRel8 => 128,      // -128 bytes
            LabelUse::JumpAbs16 => 0,       // Absolute addressing
        }
    }

    fn patch_size(self) -> CodeOffset {
        match self {
            LabelUse::JumpRel8 => 1,   // 8-bit offset
            LabelUse::JumpAbs16 => 2,  // 16-bit address
        }
    }

    fn patch(self, buffer: &mut [u8], use_offset: CodeOffset, label_offset: CodeOffset) {
        match self {
            LabelUse::JumpRel8 => {
                // Calculate relative offset from the end of the instruction
                let rel = (label_offset as i64) - (use_offset as i64) - 1;
                debug_assert!(rel >= -128 && rel <= 127, "JR offset out of range");
                buffer[0] = rel as u8;
            }
            LabelUse::JumpAbs16 => {
                // Store 16-bit address in little-endian format
                let addr = label_offset as u16;
                buffer[0] = (addr & 0xFF) as u8;        // Low byte
                buffer[1] = ((addr >> 8) & 0xFF) as u8; // High byte
            }
        }
    }

    fn supports_veneer(self) -> bool {
        // Z80 doesn't need veneers for any label use
        false
    }

    fn veneer_size(self) -> CodeOffset {
        0
    }

    fn worst_case_veneer_size() -> CodeOffset {
        0
    }

    fn generate_veneer(self, _: &mut [u8], _: CodeOffset) -> (CodeOffset, LabelUse) {
        panic!("Z80 does not support veneers")
    }

    fn from_reloc(_reloc: Reloc, _addend: Addend) -> Option<Self> {
        // TODO: Map relocation types to LabelUse variants
        None
    }
}

// Z80 ABI Machine Spec is defined in ../abi.rs
pub use crate::isa::z80::abi::Z80ABIMachineSpec;

//=============================================================================
// Addressing mode type

/// Addressing mode for Z80 memory operations
#[derive(Clone, Debug)]
pub enum Amode {
    /// Register indirect: (HL), (BC), (DE)
    RegIndirect { base: Reg },

    /// Indexed with displacement: (IX+d), (IY+d)
    IndexedDisp { base: Reg, disp: i8 },

    /// Direct/Absolute addressing: (nn)
    Absolute { addr: u16 },
}

impl From<Reg> for Amode {
    fn from(base: Reg) -> Self {
        Amode::RegIndirect { base }
    }
}
