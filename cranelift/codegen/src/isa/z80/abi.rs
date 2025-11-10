//! Z80 ABI implementation.

use crate::ir;
use crate::ir::Signature;
use crate::isa::z80::inst::Inst;
use crate::isa::z80::settings as z80_settings;
use crate::isa;
use crate::machinst::*;
use crate::settings;
use crate::CodegenResult;
use alloc::vec::Vec;
use regalloc2::{MachineEnv, PRegSet};
use smallvec::SmallVec;

/// Z80-specific ABI behavior
pub struct Z80ABIMachineSpec;

impl IsaFlags for z80_settings::Flags {}

impl ABIMachineSpec for Z80ABIMachineSpec {
    type I = Inst;
    type F = z80_settings::Flags;

    const STACK_ARG_RET_SIZE_LIMIT: u32 = 64 * 1024;

    fn word_bits() -> u32 {
        // Z80 is an 8-bit architecture
        8
    }

    fn stack_align(_call_conv: isa::CallConv) -> u32 {
        // Z80 has no specific stack alignment requirements
        1
    }

    fn compute_arg_locs(
        _call_conv: isa::CallConv,
        _flags: &settings::Flags,
        _params: &[ir::AbiParam],
        _args_or_rets: ArgsOrRets,
        _add_ret_area_ptr: bool,
        _args: ArgsAccumulator,
    ) -> CodegenResult<(u32, Option<usize>)> {
        todo!("Z80 compute_arg_locs")
    }

    fn gen_load_stack(_mem: StackAMode, _into_reg: Writable<Reg>, _ty: ir::Type) -> Self::I {
        todo!("Z80 gen_load_stack")
    }

    fn gen_store_stack(_mem: StackAMode, _from_reg: Reg, _ty: ir::Type) -> Self::I {
        todo!("Z80 gen_store_stack")
    }

    fn gen_move(to_reg: Writable<Reg>, from_reg: Reg, ty: ir::Type) -> Self::I {
        Inst::gen_move(to_reg, from_reg, ty)
    }

    fn gen_extend(
        _to_reg: Writable<Reg>,
        _from_reg: Reg,
        _is_signed: bool,
        _from_bits: u8,
        _to_bits: u8,
    ) -> Self::I {
        todo!("Z80 gen_extend")
    }

    fn gen_args(_args: Vec<ArgPair>) -> Self::I {
        todo!("Z80 gen_args")
    }

    fn gen_rets(_rets: Vec<RetPair>) -> Self::I {
        todo!("Z80 gen_rets")
    }

    fn gen_add_imm(
        _call_conv: isa::CallConv,
        _into_reg: Writable<Reg>,
        _from_reg: Reg,
        _imm: u32,
    ) -> SmallInstVec<Self::I> {
        todo!("Z80 gen_add_imm")
    }

    fn gen_stack_lower_bound_trap(_limit_reg: Reg) -> SmallInstVec<Self::I> {
        todo!("Z80 gen_stack_lower_bound_trap")
    }

    fn gen_get_stack_addr(_mem: StackAMode, _into_reg: Writable<Reg>) -> Self::I {
        todo!("Z80 gen_get_stack_addr")
    }

    fn get_stacklimit_reg(_call_conv: isa::CallConv) -> Reg {
        todo!("Z80 get_stacklimit_reg")
    }

    fn gen_load_base_offset(_into_reg: Writable<Reg>, _base: Reg, _offset: i32, _ty: ir::Type) -> Self::I {
        todo!("Z80 gen_load_base_offset")
    }

    fn gen_store_base_offset(_base: Reg, _offset: i32, _from_reg: Reg, _ty: ir::Type) -> Self::I {
        todo!("Z80 gen_store_base_offset")
    }

    fn gen_sp_reg_adjust(_amount: i32) -> SmallInstVec<Self::I> {
        todo!("Z80 gen_sp_reg_adjust")
    }

    fn compute_frame_layout(
        _call_conv: isa::CallConv,
        _flags: &settings::Flags,
        _sig: &Signature,
        _regs: &[Writable<RealReg>],
        _is_leaf: bool,
        _incoming_args_size: u32,
        _tail_args_size: u32,
        _stackslots_size: u32,
        _fixed_frame_storage_size: u32,
        _outgoing_args_size: u32,
    ) -> FrameLayout {
        // TODO: Implement proper frame layout computation
        todo!("Z80 compute_frame_layout");
        FrameLayout {
            word_bytes: Self::word_bytes(),
            incoming_args_size: 0,
            tail_args_size: 0,
            setup_area_size: 0,
            clobber_size: 0,
            fixed_frame_storage_size: 0,
            outgoing_args_size: 0,
            stackslots_size: 0,
            clobbered_callee_saves: vec![],
        }
    }

    fn gen_prologue_frame_setup(
        _call_conv: isa::CallConv,
        _flags: &settings::Flags,
        _isa_flags: &Self::F,
        _frame_layout: &FrameLayout,
    ) -> SmallInstVec<Self::I> {
        todo!("Z80 gen_prologue_frame_setup")
    }

    fn gen_epilogue_frame_restore(
        _call_conv: isa::CallConv,
        _flags: &settings::Flags,
        _isa_flags: &Self::F,
        _frame_layout: &FrameLayout,
    ) -> SmallInstVec<Self::I> {
        todo!("Z80 gen_epilogue_frame_restore")
    }

    fn gen_return(
        _call_conv: isa::CallConv,
        _isa_flags: &Self::F,
        _frame_layout: &FrameLayout,
    ) -> SmallInstVec<Self::I> {
        todo!("Z80 gen_return")
    }

    fn gen_probestack(_insts: &mut SmallInstVec<Self::I>, _frame_size: u32) {
        todo!("Z80 gen_probestack")
    }

    fn gen_inline_probestack(
        _insts: &mut SmallInstVec<Self::I>,
        _call_conv: isa::CallConv,
        _frame_size: u32,
        _guard_size: u32,
    ) {
        todo!("Z80 gen_inline_probestack")
    }

    fn gen_clobber_save(
        _call_conv: isa::CallConv,
        _flags: &settings::Flags,
        _frame_layout: &FrameLayout,
    ) -> SmallVec<[Self::I; 16]> {
        todo!("Z80 gen_clobber_save")
    }

    fn gen_clobber_restore(
        _call_conv: isa::CallConv,
        _flags: &settings::Flags,
        _frame_layout: &FrameLayout,
    ) -> SmallVec<[Self::I; 16]> {
        todo!("Z80 gen_clobber_restore")
    }

    fn gen_memcpy<F: FnMut(ir::Type) -> Writable<Reg>>(
        _call_conv: isa::CallConv,
        _dst: Reg,
        _src: Reg,
        _size: usize,
        _alloc_tmp: F,
    ) -> SmallVec<[Self::I; 8]> {
        todo!("Z80 gen_memcpy")
    }

    fn get_number_of_spillslots_for_value(
        _rc: RegClass,
        _target_vector_bytes: u32,
        _isa_flags: &Self::F,
    ) -> u32 {
        todo!("Z80 get_number_of_spillslots_for_value")
    }

    fn get_machine_env(_flags: &settings::Flags, _call_conv: isa::CallConv) -> &MachineEnv {
        use std::sync::OnceLock;

        static MACHINE_ENV: OnceLock<MachineEnv> = OnceLock::new();
        MACHINE_ENV.get_or_init(|| {
            MachineEnv {
                preferred_regs_by_class: [
                    vec![], // TODO: Int registers
                    vec![], // Float registers - Z80 has none
                    vec![], // Vector registers - Z80 has none
                ],
                non_preferred_regs_by_class: [
                    vec![], // Int
                    vec![], // Float
                    vec![], // Vector
                ],
                scratch_by_class: [None, None, None],
                fixed_stack_slots: vec![],
            }
        })
    }

    fn get_regs_clobbered_by_call(
        _call_conv_of_callee: isa::CallConv,
        _is_exception: bool,
    ) -> PRegSet {
        PRegSet::empty()
    }

    fn get_ext_mode(
        _call_conv: isa::CallConv,
        _specified: ir::ArgumentExtension,
    ) -> ir::ArgumentExtension {
        ir::ArgumentExtension::None
    }

    fn retval_temp_reg(_call_conv_of_callee: isa::CallConv) -> Writable<Reg> {
        todo!("Z80 retval_temp_reg")
    }
}
