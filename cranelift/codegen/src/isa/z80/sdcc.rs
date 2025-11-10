#[cfg(feature = "enable-serde")]
use serde_derive::{Deserialize, Serialize};

/// Calling convention identifiers.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
pub enum SdccCallConv {
    /// Z80 SDCC calling convention, version 1
    /// `__sdcccall(1)` in SDCC
    ///
    /// # Parameters (ADEHL registers are used)
    /// For functions that have variable arguments: all parameters are passed on the stack, right-to-left.
    /// For Functions that do not have variable arguments: 
    /// the first parameter is passed in a if it has 8 bits. 
    /// If it has 16 bits it is passed in hl. 
    /// If it has 32 bits, it is passed in hlde. 
    /// If the first parameter is in a, and the second has 8 bits, it is passed in l; 
    /// if the first is in hl or hlde, and the second has 8 bits, it is passed in a; 
    /// if the first is in a, and the second has 16 bits, it is passed in hl; 
    /// all other parameters are passed on the stack, right-to-left. 
    /// Independent of their size, struct / union parameters and all following parameters are always passed on the stack.
    ///
    /// # Return Value (ADEHL registers are used)
    /// - 8-bit return value in A.
    /// - 16-bit return value in HL.
    /// - 24-bit return value in LDE.
    /// - 32-bit return value in HLDE.
    /// Larger return values (as well as struct and union independent of their size)
    /// jre passed in a memory in a location specified by the caller through the hidden pointer
    /// argument.
    ///
    /// # Stack Cleanup
    /// After the call, the stack parameters are cleaned up by the caller, with the following
    /// exceptions: functions that do not have variable arguments and return void or a type of at most 16 bits, or have both a first
    /// parameter of type float and a return value of type float.
    Z80V1,

    /// SM83 SDCC calling convention, version 0
    /// `__sdcccall(0)` in SDCC
    ///
    /// # Parameters
    /// All parameters are passed on the stack.
    ///
    /// # Return value (DEHL registers are used)
    /// - 8-bit return value in E.
    /// - 16-bit return value in DE.
    /// - 32-bit return value in HLDE.
    /// Larger return values (as well as struct and union independent of their size)
    /// are passed in a memory in a location specified by the caller through the hidden pointer
    ///
    /// # Stack Cleanup
    /// After the call, the stack parameters are cleaned up by the caller.
    Sm83V0,

    /// SM84 SDCC calling convention, version 1
    /// `__sdcccall(1)` in SDCC
    ///
    /// # Parameters (ABCDE registers are used)
    /// For Functions that do not have variable arguments: the first parameter is passed in a if it has 8 bits. 
    /// If it has 16 bits it is passed in de. 
    /// If it has 32 bits, it is passed in debc. 
    /// If the first parameter is in a, and the second has 8 bits, it is passed in e; 
    /// if the first is in bc or debc, and the second has 8 bits, it is passed in a; 
    /// if the first is passed in a, and the second has 16 bits, it is passed in bc; 
    /// if the first is passed in de, and the second has 16 bits, it is passed in bc; 
    /// all other parameters are passed on the stack, right-to-left. 
    /// Independent of their size, struct / union parameters and all following parameters are
    /// always passed on the stack.
    ///
    /// # Return Value (ADEHL registers are used)
    /// - 8-bit return value in A.
    /// - 16-bit return value in HL.
    /// - 24-bit return value in LDE.
    /// - 32-bit return value in HLDE.
    /// Larger return values (as well as struct and union independent of their size)
    /// jre passed in a memory in a location specified by the caller through the hidden pointer
    /// argument.
    ///
    /// # Stack Cleanup
    /// After the call, the stack parameters are cleaned up by the caller, with the following
    /// exceptions: functions that do not have variable arguments and return void or a type of at most 16 bits, or have both a first
    /// parameter of type float and a return value of type float.
    Sm83V1,

    // Not yet implemented calling conventions
    // SmallC,
    // DynamicC,
    // Z80V0,
    // Z80V0Callee,
    // RabbitV0,
    // RabbitV1,
}
