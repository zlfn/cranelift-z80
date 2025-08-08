use core::fmt;
use std::vec::Vec;

use cranelift_control::ControlPlane;
use target_lexicon::Triple;

use crate::ir;
use crate::isa::Builder as IsaBuilder;
use crate::isa::z80::settings as z80_settings;
use crate::machinst::CompiledCode;
use crate::settings::{self as shared_settings, Flags};
use crate::{
    CodegenResult, dominator_tree::DominatorTree, ir::Function, machinst::CompiledCodeStencil,
};

use super::{FunctionAlignment, IsaFlagsHashKey, OwnedTargetIsa, TargetIsa};

pub mod settings;

pub(crate) struct Z80Backend {
    triple: Triple,
    flags: Flags,
    z80_flags: z80_settings::Flags,
}

impl Z80Backend {
    /// Create a new Z80 backend with the given (shared) flags.
    fn new_with_flags(triple: Triple, flags: Flags, z80_flags: z80_settings::Flags) -> Self {
        Self {
            triple,
            flags,
            z80_flags,
        }
    }
}

impl TargetIsa for Z80Backend {
    fn compile_function(
        &self,
        func: &Function,
        domtree: &DominatorTree,
        want_disasm: bool,
        ctrl_plane: &mut ControlPlane,
    ) -> CodegenResult<CompiledCodeStencil> {
        todo!()
    }

    fn flags(&self) -> &Flags {
        &self.flags
    }

    fn isa_flags(&self) -> Vec<shared_settings::Value> {
        self.z80_flags.iter().collect()
    }

    fn isa_flags_hash_key(&self) -> IsaFlagsHashKey<'_> {
        IsaFlagsHashKey(self.z80_flags.hash_key())
    }

    fn dynamic_vector_bytes(&self, _dyn_ty: crate::ir::Type) -> u32 {
        todo!()
    }

    fn name(&self) -> &'static str {
        "z80"
    }

    fn triple(&self) -> &Triple {
        &self.triple
    }

    fn text_section_builder(
        &self,
        num_labeled_funcs: usize,
    ) -> std::prelude::v1::Box<dyn crate::TextSectionBuilder> {
        todo!()
    }

    fn function_alignment(&self) -> FunctionAlignment {
        todo!()
    }

    fn page_size_align_log2(&self) -> u8 {
        todo!()
    }

    fn pretty_print_reg(&self, reg: crate::Reg, size: u8) -> std::string::String {
        todo!()
    }

    fn has_native_fma(&self) -> bool {
        // FMA? On a Z80? Not likely.
        false
    }

    fn has_round(&self) -> bool {
        // Z80 lacks floating-point support, so no rounding either.
        false
    }

    fn has_x86_blendv_lowering(&self, ty: crate::ir::Type) -> bool {
        false
    }

    fn has_x86_pshufb_lowering(&self) -> bool {
        false
    }

    fn has_x86_pmulhrsw_lowering(&self) -> bool {
        false
    }

    fn has_x86_pmaddubsw_lowering(&self) -> bool {
        false
    }

    fn default_argument_extension(&self) -> crate::ir::ArgumentExtension {
        ir::ArgumentExtension::None
    }

    #[cfg(feature = "unwind")]
    fn emit_unwind_info(
        &self,
        result: &CompiledCode,
        kind: crate::isa::unwind::UnwindInfoKind,
    ) -> CodegenResult<Option<crate::isa::unwind::UnwindInfo>> {
        unimplemented!("Z80 does not support unwinding");
    }

    #[cfg(feature = "unwind")]
    fn create_systemv_cie(&self) -> Option<gimli::write::CommonInformationEntry> {
        // Z80 thinks System V is just Saturn V spelled wrong.
        // (Z80 launched in 1976, Unix System V 1983)
        None
    }

    #[cfg(feature = "unwind")]
    fn map_regalloc_reg_to_dwarf(
        &self,
        _: crate::machinst::Reg,
    ) -> Result<u16, super::unwind::systemv::RegisterMappingError> {
        unimplemented!("Z80 does not support unwinding");
    }

    #[cfg(feature = "disas")]
    fn to_capstone(&self) -> Result<capstone::Capstone, capstone::Error> {
        todo!()
    }
}

impl fmt::Display for Z80Backend {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Z80Backend")
            .field("name", &self.name())
            .field("triple", &self.triple())
            .field("flags", &format!("{}", self.flags()))
            .finish()
    }
}

/// Create a new `isa::Builder`.
pub(crate) fn isa_builder(triple: Triple) -> IsaBuilder {
    IsaBuilder {
        triple,
        setup: z80_settings::builder(),
        constructor: isa_constructor,
    }
}

fn isa_constructor(
    triple: Triple,
    shared_flags: Flags,
    builder: &shared_settings::Builder,
) -> CodegenResult<OwnedTargetIsa> {
    let isa_flags = z80_settings::Flags::new(&shared_flags, builder);
    let backend = Z80Backend::new_with_flags(triple, shared_flags, isa_flags);
    Ok(backend.wrapped())
}
