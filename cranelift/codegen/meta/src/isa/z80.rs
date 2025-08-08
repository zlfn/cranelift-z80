use crate::cdsl::{isa::TargetIsa, settings::SettingGroupBuilder};

pub(crate) fn define() -> TargetIsa {
    let mut settings = SettingGroupBuilder::new("z80");

    settings.add_bool(
        "has_index_registers",
        "Has IX / IY index registers and indirect addressing using displacement (DD, FD prefixed instructions)",
        "16-bit index registers used for indirect addressing with displacement.",
        true,
    );

    settings.add_bool(
        "has_shadow_registers",
        "Has AF' / BC' / DE' / HL' shadow registers and EX / EXX instructions",
        "Shadow registers that can be used to save and restore the state of the main registers.",
        true,
    );

    settings.add_bool(
        "has_sign_parity_overflow_flags",
        "Has PO / PE / P / M flags in conditional instructions",
        "Flags that indicate the sign, parity, and overflow status of the last operation.",
        true,
    );

    settings.add_bool(
        "has_ed_prefixed_instructions",
        "Has ED prefixed instructions (e.g., IN, OUT, LD A I, LD A R, ADC HL, SDC HL, RLD, RRD etc.)",
        "ED prefix operations that provide additional 16-bit memory accesses and arithmetic instructions \
          and input/output instructions.",
        true,
    );

    settings.add_bool(
        "has_djnz",
        "Has DJNZ instruction",
        "DJNZ (Decrement and Jump if Not Zero) instruction that decrements the B register and jumps \
          to a specified address if the result is not zero.",
        true,
    );

    settings.add_bool(
        "has_z80_undocumented_instructions",
        "Has Z80 undocumented instructions (e.g., SLL)",
        "Undocumented instructions that are specific to the Z80 processor, such as SLL (Shift Left Logical).\
          These instructions may not be present in emulators or other Z80-compatible processors.",
        true,
    );

    settings.add_bool(
        "has_z180_extended_instructions",
        "Z180 extended instructions (e.g., MLT, OTIM, OTDM, etc.)",
        "Instructions specific to the Z180 processor that extend the capabilities of the Z80.",
        false,
    );

    settings.add_bool(
        "has_sm83_extended_instructions",
        "Has SM83 extended instructions (e.g., SWAP, HL+, etc.)",
        "Extended instructions specific to the Sharp SM83 (Game Boy) processor\n\
          * SWAP r (swap nibbles in a register)\n\
          * HL-, HL+ (autoincrement HL register)\n\
          * ADD SP dd (add signed displacement to SP register)",
        false,
    );

    settings.add_bool(
        "has_z80n_extended_instructions",
        "Has Z80N (Next-specific Z80) extended instructions (e.g., SWAP, MIRR, OTIB, etc.)",
        "Extended instructions specific to the Z80N (ZX Spectrum Next) processor",
        false,
    );

    TargetIsa::new("z80", settings.build())
}
