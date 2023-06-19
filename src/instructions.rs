use std::collections::HashMap;
use lazy_static::lazy_static;


pub struct Instruction<'a> {
    pub name: &'a str,
    pub gas_cost: u32,
    pub operands: usize
}

impl Instruction<'_> {
    fn new(name: &str, gas_cost: u32, operands: usize) -> Instruction {
        return Instruction {
            name: name,
            gas_cost: gas_cost,
            operands: operands
        };
    }
}

lazy_static! {
    pub static ref INSTRUCTIONS: HashMap<u8, Instruction<'static>> = {
        let mut instructions = HashMap::new();
        instructions.insert(0x00, Instruction::new("STOP", 0, 0));
        instructions.insert(0x01, Instruction::new("ADD", 3, 0));
        instructions.insert(0x02, Instruction::new("MUL", 5, 0));
        instructions.insert(0x03, Instruction::new("SUB", 3, 0));
        instructions.insert(0x04, Instruction::new("DIV", 5, 0));
        instructions.insert(0x05, Instruction::new("SDIV", 5, 0));
        instructions.insert(0x06, Instruction::new("MOD", 5, 0));
        instructions.insert(0x07, Instruction::new("SMOD", 5, 0));
        instructions.insert(0x08, Instruction::new("ADDMOD", 8, 0));
        instructions.insert(0x09, Instruction::new("MULMOD", 8, 0));
        instructions.insert(0x0A, Instruction::new("EXP", 10, 0));
        instructions.insert(0x0B, Instruction::new("SIGNEXTEND", 5, 0));
        instructions.insert(0x10, Instruction::new("LT", 3, 0));
        instructions.insert(0x11, Instruction::new("GT", 3, 0));
        instructions.insert(0x12, Instruction::new("SLT", 3, 0));
        instructions.insert(0x13, Instruction::new("SGT", 3, 0));
        instructions.insert(0x14, Instruction::new("EQ", 3, 0));
        instructions.insert(0x15, Instruction::new("ISZERO", 3, 0));
        instructions.insert(0x16, Instruction::new("AND", 3, 0));
        instructions.insert(0x17, Instruction::new("OR", 3, 0));
        instructions.insert(0x18, Instruction::new("XOR", 3, 0));
        instructions.insert(0x19, Instruction::new("NOT", 3, 0));
        instructions.insert(0x1A, Instruction::new("BYTE", 3, 0));
        instructions.insert(0x1B, Instruction::new("SHL", 3, 0));
        instructions.insert(0x1C, Instruction::new("SHR", 3, 0));
        instructions.insert(0x1D, Instruction::new("SAR", 3, 0));
        instructions.insert(0x20, Instruction::new("SHA3", 30, 0));
        instructions.insert(0x30, Instruction::new("ADDRESS", 2, 0));
        instructions.insert(0x31, Instruction::new("BALANCE", 100, 0));
        instructions.insert(0x32, Instruction::new("ORIGIN", 2, 0));
        instructions.insert(0x33, Instruction::new("CALLER", 2, 0));
        instructions.insert(0x34, Instruction::new("CALLVALUE", 2, 0));
        instructions.insert(0x35, Instruction::new("CALLDATALOAD", 3, 0));
        instructions.insert(0x36, Instruction::new("CALLDATASIZE", 2, 0));
        instructions.insert(0x37, Instruction::new("CALLDATACOPY", 3, 0));
        instructions.insert(0x38, Instruction::new("CODESIZE", 2, 0));
        instructions.insert(0x39, Instruction::new("CODECOPY", 3, 0));
        instructions.insert(0x3A, Instruction::new("GASPRICE", 2, 0));
        instructions.insert(0x3B, Instruction::new("EXTCODESIZE", 100, 0));
        instructions.insert(0x3C, Instruction::new("EXTCODECOPY", 100, 0));
        instructions.insert(0x3D, Instruction::new("RETURNDATASIZE", 2, 0));
        instructions.insert(0x3E, Instruction::new("RETURNDATACOPY", 3, 0));
        instructions.insert(0x3F, Instruction::new("EXTCODEHASH", 100, 0));
        instructions.insert(0x40, Instruction::new("BLOCKHASH", 20, 0));
        instructions.insert(0x41, Instruction::new("COINBASE", 2, 0));
        instructions.insert(0x42, Instruction::new("TIMESTAMP", 2, 0));
        instructions.insert(0x43, Instruction::new("NUMBER", 2, 0));
        instructions.insert(0x44, Instruction::new("DIFFICULTY", 2, 0));
        instructions.insert(0x45, Instruction::new("GASLIMIT", 2, 0));
        instructions.insert(0x46, Instruction::new("CHAINID", 2, 0));
        instructions.insert(0x47, Instruction::new("SELFBALANCE", 5, 0));
        instructions.insert(0x48, Instruction::new("BASEFEE", 2, 0));
        instructions.insert(0x50, Instruction::new("POP", 2, 0));
        instructions.insert(0x51, Instruction::new("MLOAD", 3, 0));
        instructions.insert(0x52, Instruction::new("MSTORE", 3, 0));
        instructions.insert(0x53, Instruction::new("MSTORE8", 3, 0));
        instructions.insert(0x54, Instruction::new("SLOAD", 100, 0));
        instructions.insert(0x55, Instruction::new("SSTORE", 100, 0));
        instructions.insert(0x56, Instruction::new("JUMP", 8, 0));
        instructions.insert(0x57, Instruction::new("JUMPI", 10, 0));
        instructions.insert(0x58, Instruction::new("PC", 2, 0));
        instructions.insert(0x59, Instruction::new("MSIZE", 2, 0));
        instructions.insert(0x5A, Instruction::new("GAS", 2, 0));
        instructions.insert(0x5B, Instruction::new("JUMPDEST", 1, 0));
        instructions.insert(0x5F, Instruction::new("PUSH0", 2, 0));
        instructions.insert(0x60, Instruction::new("PUSH1", 3, 1));
        instructions.insert(0x61, Instruction::new("PUSH2", 3, 2));
        instructions.insert(0x62, Instruction::new("PUSH3", 3, 3));
        instructions.insert(0x63, Instruction::new("PUSH4", 3, 4));
        instructions.insert(0x64, Instruction::new("PUSH5", 3, 5));
        instructions.insert(0x65, Instruction::new("PUSH6", 3, 6));
        instructions.insert(0x66, Instruction::new("PUSH7", 3, 7));
        instructions.insert(0x67, Instruction::new("PUSH8", 3, 8));
        instructions.insert(0x68, Instruction::new("PUSH9", 3, 9));
        instructions.insert(0x69, Instruction::new("PUSH10", 3, 10));
        instructions.insert(0x6A, Instruction::new("PUSH11", 3, 11));
        instructions.insert(0x6B, Instruction::new("PUSH12", 3, 12));
        instructions.insert(0x6C, Instruction::new("PUSH13", 3, 13));
        instructions.insert(0x6D, Instruction::new("PUSH14", 3, 14));
        instructions.insert(0x6E, Instruction::new("PUSH15", 3, 15));
        instructions.insert(0x6F, Instruction::new("PUSH16", 3, 16));
        instructions.insert(0x70, Instruction::new("PUSH17", 3, 17));
        instructions.insert(0x71, Instruction::new("PUSH18", 3, 18));
        instructions.insert(0x72, Instruction::new("PUSH19", 3, 19));
        instructions.insert(0x73, Instruction::new("PUSH20", 3, 20));
        instructions.insert(0x74, Instruction::new("PUSH21", 3, 21));
        instructions.insert(0x75, Instruction::new("PUSH22", 3, 22));
        instructions.insert(0x76, Instruction::new("PUSH23", 3, 23));
        instructions.insert(0x77, Instruction::new("PUSH24", 3, 24));
        instructions.insert(0x78, Instruction::new("PUSH25", 3, 25));
        instructions.insert(0x79, Instruction::new("PUSH26", 3, 26));
        instructions.insert(0x7A, Instruction::new("PUSH27", 3, 27));
        instructions.insert(0x7B, Instruction::new("PUSH28", 3, 28));
        instructions.insert(0x7C, Instruction::new("PUSH29", 3, 29));
        instructions.insert(0x7D, Instruction::new("PUSH30", 3, 30));
        instructions.insert(0x7E, Instruction::new("PUSH31", 3, 31));
        instructions.insert(0x7F, Instruction::new("PUSH32", 3, 32));
        instructions.insert(0x80, Instruction::new("DUP1", 3, 0));
        instructions.insert(0x81, Instruction::new("DUP2", 3, 0));
        instructions.insert(0x82, Instruction::new("DUP3", 3, 0));
        instructions.insert(0x83, Instruction::new("DUP4", 3, 0));
        instructions.insert(0x84, Instruction::new("DUP5", 3, 0));
        instructions.insert(0x85, Instruction::new("DUP6", 3, 0));
        instructions.insert(0x86, Instruction::new("DUP7", 3, 0));
        instructions.insert(0x87, Instruction::new("DUP8", 3, 0));
        instructions.insert(0x88, Instruction::new("DUP9", 3, 0));
        instructions.insert(0x89, Instruction::new("DUP10", 3, 0));
        instructions.insert(0x8A, Instruction::new("DUP11", 3, 0));
        instructions.insert(0x8B, Instruction::new("DUP12", 3, 0));
        instructions.insert(0x8C, Instruction::new("DUP13", 3, 0));
        instructions.insert(0x8D, Instruction::new("DUP14", 3, 0));
        instructions.insert(0x8E, Instruction::new("DUP15", 3, 0));
        instructions.insert(0x8F, Instruction::new("DUP16", 3, 0));
        instructions.insert(0x90, Instruction::new("SWAP1", 3, 0));
        instructions.insert(0x91, Instruction::new("SWAP2", 3, 0));
        instructions.insert(0x92, Instruction::new("SWAP3", 3, 0));
        instructions.insert(0x93, Instruction::new("SWAP4", 3, 0));
        instructions.insert(0x94, Instruction::new("SWAP5", 3, 0));
        instructions.insert(0x95, Instruction::new("SWAP6", 3, 0));
        instructions.insert(0x96, Instruction::new("SWAP7", 3, 0));
        instructions.insert(0x97, Instruction::new("SWAP8", 3, 0));
        instructions.insert(0x98, Instruction::new("SWAP9", 3, 0));
        instructions.insert(0x99, Instruction::new("SWAP10", 3, 0));
        instructions.insert(0x9A, Instruction::new("SWAP11", 3, 0));
        instructions.insert(0x9B, Instruction::new("SWAP12", 3, 0));
        instructions.insert(0x9C, Instruction::new("SWAP13", 3, 0));
        instructions.insert(0x9D, Instruction::new("SWAP14", 3, 0));
        instructions.insert(0x9E, Instruction::new("SWAP15", 3, 0));
        instructions.insert(0x9F, Instruction::new("SWAP16", 3, 0));
        instructions.insert(0xA0, Instruction::new("LOG0", 375, 0));
        instructions.insert(0xA1, Instruction::new("LOG1", 750, 0));
        instructions.insert(0xA2, Instruction::new("LOG2", 1125, 0));
        instructions.insert(0xA3, Instruction::new("LOG3", 1500, 0));
        instructions.insert(0xA4, Instruction::new("LOG4", 1875, 0));
        instructions.insert(0xF0, Instruction::new("CREATE", 32000, 0));
        instructions.insert(0xF1, Instruction::new("CALL", 100, 0));
        instructions.insert(0xF2, Instruction::new("CALLCODE", 100, 0));
        instructions.insert(0xF3, Instruction::new("RETURN", 0, 0));
        instructions.insert(0xF4, Instruction::new("DELEGAATECALL", 100, 0));
        instructions.insert(0xF5, Instruction::new("CREATE2", 32000, 0));
        instructions.insert(0xFA, Instruction::new("STATICCALL", 100, 0));
        instructions.insert(0xFD, Instruction::new("REVERT", 0, 0));
        instructions.insert(0xFE, Instruction::new("INVALID", 0, 0));
        instructions.insert(0xFF, Instruction::new("SELFDESTRUCT", 5000, 0));

        return instructions;
    };
}
