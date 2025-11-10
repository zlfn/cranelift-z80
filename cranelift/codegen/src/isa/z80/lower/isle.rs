pub mod generated_code;

// Types that the generated ISLE code uses via `use super::*`.
use crate::ir::{condcodes::*, immediates::*, types::*, *};
use crate::isa::z80::*;
use crate::machinst::{
    CallArgList, CallInfo, CallRetList, MachInst, Reg, VCodeConstant, VCodeConstantData,
    abi::{ArgPair, RetPair, StackAMode},
    isle::*,
};
use regalloc2::PReg;

type Unit = ();
type VecArgPair = Vec<ArgPair>;
type VecRetPair = Vec<RetPair>;
