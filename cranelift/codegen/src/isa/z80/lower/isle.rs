pub mod generated_code;
use generated_code::MInst;

// Types that the generated ISLE code uses via `use super::*`.
use crate::ir::{condcodes::*, immediates::*, types::*, *, Inst};
use crate::isa::z80::*;
use crate::isa::z80::inst::{*, regs};
use crate::machinst::{
    CallArgList, CallInfo, CallRetList, MachInst, Reg, VCodeConstant, VCodeConstantData,
    abi::{ArgPair, RetPair, StackAMode},
    isle::*,
};
use regalloc2::PReg;

type Unit = ();
type VecArgPair = Vec<ArgPair>;
type VecRetPair = Vec<RetPair>;

pub(crate) struct Z80IsleContext<'a, 'b, I, B>
where 
    I: VCodeInst,
    B: LowerBackend,
{
    pub lower_ctx: &'a mut Lower<'b, I>,
    pub backend: &'a B,
}

impl<'a, 'b> Z80IsleContext<'a, 'b, MInst, Z80Backend> {
    fn new(
        lower_ctx: &'a mut Lower<'b, MInst>,
        backend: &'a Z80Backend,
    ) -> Self {
        Self { lower_ctx, backend }
    }

    pub(crate) fn dfg(&self) -> &crate::ir::DataFlowGraph {
        &self.lower_ctx.f.dfg
    }
}

pub(crate) fn lower(
    lower_ctx: &mut Lower<MInst>,
    backend: &Z80Backend,
    inst: Inst
) -> Option<InstOutput> {
    let mut isle_ctx = Z80IsleContext::new(lower_ctx, backend);
    todo!("constructor_lower")
    // generated_code::constructor_lower(&mut isle_ctx, inst)
}

pub(crate) fn lower_branch(
    lower_ctx: &mut Lower<MInst>,
    backend: &Z80Backend,
    branch: Inst,
    targets: &[MachLabel],
) -> Option<()> {
    let mut isle_ctx = Z80IsleContext::new(lower_ctx, backend);
    todo!("constructor_lower_branch")
    // generated_code::constructor_lower_branch(&mut isle_ctx, branch, targets)
}
