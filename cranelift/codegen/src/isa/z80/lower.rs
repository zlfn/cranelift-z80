use crate::ir::Inst as IRInst;
use crate::isa::z80::Z80Backend;
use crate::isa::z80::inst::*;
use crate::machinst::lower::*;
use crate::machinst::*;

pub mod isle;

impl LowerBackend for Z80Backend {
    type MInst = Inst;

    fn lower(&self, ctx: &mut Lower<Inst>, ir_inst: IRInst) -> Option<InstOutput> {
        isle::lower(ctx, self, ir_inst)
    }

    fn lower_branch(
        &self,
        ctx: &mut Lower<Inst>,
        ir_inst: IRInst,
        targets: &[MachLabel],
    ) -> Option<()> {
        isle::lower_branch(ctx, self, ir_inst, targets)
    }

    fn maybe_pinned_reg(&self) -> Option<Reg> {
        None
    }

    type FactFlowState = ();
}
