use ethnum::{U256, I256};
use sha3::{Digest, Keccak256};
use std::collections::HashMap;
use lazy_static::lazy_static;

use crate::context;
use crate::blockchain;

struct Instruction<'a> {
    name: &'a str,
    gas_cost: u32,
    operands: usize 
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
    static ref INSTRUCTIONS: HashMap<u8, Instruction<'static>> = {
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


pub struct EVM<'a> {
    blockchain: &'a mut blockchain::BlockChain,
    pc: usize,
    stack: Vec<U256>,
    memory: Vec<u8>,
    returndata: Vec<u8>,
    ctx: context::CTX,
    success: bool
}

impl <'a>EVM<'a> {
    pub fn new(mut blockchain: &'a mut blockchain::BlockChain, ctx: context::CTX) -> EVM {
        return EVM {
            blockchain: blockchain,
            pc: 0,
            stack: Vec::new(),
            memory: Vec::new(),
            returndata: Vec::new(),
            ctx: ctx,
            success: true
        };
    }

    pub fn run(&mut self) -> (bool, Vec<u8>) {
        let mut returndata: Vec<u8> = Vec::new();
        let mut opcode: u8;
        while self.success && self.pc < self.ctx.code.len() {
            opcode = self.ctx.code[self.pc];
            self.pc += 1;

            let instruction: &Instruction = INSTRUCTIONS.get(&opcode).unwrap();

            if instruction.operands > 0 {
                let data = &self.ctx.code[self.pc..self.pc + instruction.operands];
                let hex_data = hex::encode(data);
                println!("{} 0x{} \t// {:#02X} {:02X?}", instruction.name, hex_data, opcode, data);
            } else {
                println!("{} \t\t// {:#02X}", instruction.name, opcode);
            }

            match opcode {
                0x00 => { 
                    self.opcode_stop();
                    break;
                },
                0x01 => self.opcode_add(),
                0x02 => self.opcode_mul(),
                0x03 => self.opcode_sub(),
                0x04 => self.opcode_div(),
                0x05 => self.opcode_sdiv(),
                0x06 => self.opcode_mod(),
                0x07 => self.opcode_smod(),
                0x08 => self.opcode_addmod(),
                0x09 => self.opcode_mulmod(),
                0x0A => self.opcode_exp(),
                0x0B => self.opcode_signextend(),
                0x10 => self.opcode_lt(),
                0x11 => self.opcode_gt(),
                0x12 => self.opcode_slt(),
                0x13 => self.opcode_sgt(),
                0x14 => self.opcode_eq(),
                0x15 => self.opcode_iszero(),
                0x16 => self.opcode_and(),
                0x17 => self.opcode_or(),
                0x18 => self.opcode_xor(),
                0x19 => self.opcode_not(),
                0x1A => self.opcode_byte(),
                0x1B => self.opcode_shl(),
                0x1C => self.opcode_shr(),
                0x1D => self.opcode_sar(),
                0x20 => self.opcode_sha3(),
                0x30 => self.opcode_address(),
                0x31 => self.opcode_balance(),
                0x32 => self.opcode_origin(),
                0x33 => self.opcode_caller(),
                0x34 => self.opcode_callvalue(),
                0x35 => self.opcode_calldataload(),
                0x36 => self.opcode_calldatasize(),
                0x37 => self.opcode_calldatacopy(),
                0x38 => self.opcode_codesize(),
                0x39 => self.opcode_codecopy(),
                0x3A => self.opcode_gasprice(),
                0x3B => self.opcode_extcodesize(),
                0x3C => self.opcode_extcodecopy(),
                0x3D => self.opcode_returndatasize(),
                0x3E => self.opcode_returndatacopy(),
                0x3F => self.opcode_extcodehash(),
                0x40 => self.opcode_blockhash(),
                0x41 => self.opcode_coinbase(),
                0x42 => self.opcode_timestamp(),
                0x43 => self.opcode_number(),
                0x44 => self.opcode_difficulty(),
                0x45 => self.opcode_gaslimit(),
                0x46 => self.opcode_chainid(),
                0x47 => self.opcode_selfbalance(),
                0x48 => self.opcode_basefee(),
                0x50 => self.opcode_pop(),
                0x51 => self.opcode_mload(),
                0x52 => self.opcode_mstore(),
                0x53 => self.opcode_mstore8(),
                0x54 => self.opcode_sload(),
                0x55 => self.opcode_sstore(),
                0x56 => self.opcode_jump(),
                0x57 => self.opcode_jumpi(),
                0x58 => self.opcode_pc(),
                0x59 => self.opcode_msize(),
                0x5A => self.opcode_gas(),
                0x5B => self.opcode_jumpdest(),
                0x5F => self.opcode_push0(),
                0x60 => self.opcode_push(1),
                0x61 => self.opcode_push(2),
                0x62 => self.opcode_push(3),
                0x63 => self.opcode_push(4),
                0x64 => self.opcode_push(5),
                0x65 => self.opcode_push(6),
                0x66 => self.opcode_push(7),
                0x67 => self.opcode_push(8),
                0x68 => self.opcode_push(9),
                0x69 => self.opcode_push(10),
                0x6a => self.opcode_push(11),
                0x6b => self.opcode_push(12),
                0x6c => self.opcode_push(13),
                0x6d => self.opcode_push(14),
                0x6e => self.opcode_push(15),
                0x6f => self.opcode_push(16),
                0x70 => self.opcode_push(17),
                0x71 => self.opcode_push(18),
                0x72 => self.opcode_push(19),
                0x73 => self.opcode_push(20),
                0x74 => self.opcode_push(21),
                0x75 => self.opcode_push(22),
                0x76 => self.opcode_push(23),
                0x77 => self.opcode_push(24),
                0x78 => self.opcode_push(25),
                0x79 => self.opcode_push(26),
                0x7a => self.opcode_push(27),
                0x7b => self.opcode_push(28),
                0x7c => self.opcode_push(29),
                0x7d => self.opcode_push(30),
                0x7e => self.opcode_push(31),
                0x7f => self.opcode_push(32),
                0x80 => self.opcode_dup(1),
                0x81 => self.opcode_dup(2),
                0x82 => self.opcode_dup(3),
                0x83 => self.opcode_dup(4),
                0x84 => self.opcode_dup(5),
                0x85 => self.opcode_dup(6),
                0x86 => self.opcode_dup(7),
                0x87 => self.opcode_dup(8),
                0x88 => self.opcode_dup(9),
                0x89 => self.opcode_dup(10),
                0x8A => self.opcode_dup(11),
                0x8B => self.opcode_dup(12),
                0x8C => self.opcode_dup(13),
                0x8D => self.opcode_dup(14),
                0x8E => self.opcode_dup(15),
                0x8F => self.opcode_dup(16),
                0x90 => self.opcode_swap(1),
                0x91 => self.opcode_swap(2),
                0x92 => self.opcode_swap(3),
                0x93 => self.opcode_swap(4),
                0x94 => self.opcode_swap(5),
                0x95 => self.opcode_swap(6),
                0x96 => self.opcode_swap(7),
                0x97 => self.opcode_swap(8),
                0x98 => self.opcode_swap(9),
                0x99 => self.opcode_swap(10),
                0x9A => self.opcode_swap(11),
                0x9B => self.opcode_swap(12),
                0x9C => self.opcode_swap(13),
                0x9D => self.opcode_swap(14),
                0x9E => self.opcode_swap(15),
                0x9F => self.opcode_swap(16),
                0xA0 => self.opcode_log0(),
                0xA1 => self.opcode_log1(),
                0xA2 => self.opcode_log2(),
                0xA3 => self.opcode_log3(),
                0xA4 => self.opcode_log4(),
                0xF0 => self.opcode_create(),
                0xF1 => self.opcode_call(),
                0xF2 => self.opcode_callcode(),
                0xF3 => {
                    returndata = self.opcode_return();
                    break;
                },

                0xF4 => self.opcode_delegatecall(),
                0xF5 => self.opcode_create2(),
                0xFA => self.opcode_staticcall(),
                0xFD => {
                    returndata = self.opcode_revert();
                    self.success = false;
                },
                0xFE => {
                    returndata = self.opcode_invalid();
                    self.success = false;
                },
                0xFF => self.opcode_selfdestruct(),
                _ => {
                    println!("error");
                    self.pc += 1;
                }
            }
        }

        return (self.success, returndata);
    }

    pub fn print_stack(&self) {
        println!("-- Stack --");
        println!("{:?}", self.stack);
    }

    pub fn print_memory(&self) {
        println!("-- Memory --");
        println!("{:?}", self.memory);
    }

    fn stack_pop(&mut self) -> U256 {
        return self.stack.pop().unwrap();
    }

    fn stack_push(&mut self, val: U256) {
        self.stack.push(val);
    }

    fn memory_load(&mut self, offset: usize, size: usize) -> Vec<u8> {
        if self.memory.len() < offset + size {
            self.memory.resize(offset + size, 0);
        }
        return self.memory[offset..offset + size].to_vec();
    }

    fn memory_store(&mut self, offset: usize, data: Vec<u8>) {
        let size: usize = data.len();
        if self.memory.len() < offset + size {
            self.memory.resize(offset + size, 0);
        }

        let i: usize;
        for i in 0..size {
            self.memory[offset + i] = data[i];
        }
    }

    fn opcode_stop(&self) {
    }
    
    fn opcode_add(&mut self) {
        let a: U256 = self.stack_pop();
        let b: U256 = self.stack_pop();

        let result: U256 = a + b;
        self.stack_push(result);
    }
    
    fn opcode_mul(&mut self) {
        let a: U256 = self.stack_pop();
        let b: U256 = self.stack_pop();
    
        let result: U256 = a * b;
        self.stack_push(result);
    }

    fn opcode_sub(&mut self) {
        let a: U256 = self.stack_pop();
        let b: U256 = self.stack_pop();

        let result: U256 = a - b;
        self.stack_push(result);
    }

    fn opcode_div(&mut self) {
        let a: U256 = self.stack_pop();
        let b: U256 = self.stack_pop();

        let result: U256 = a / b;
        self.stack_push(result);
    }

    fn opcode_sdiv(&mut self) {
        let a: U256 = self.stack_pop();
        let b: U256 = self.stack_pop();

        let result: U256 = a / b;
        self.stack_push(result);
    }

    fn opcode_mod(&mut self) {
        let a: U256 = self.stack_pop();
        let b: U256 = self.stack_pop();

        let result: U256 = a % b;
        self.stack_push(result);
    }

    fn opcode_smod(&mut self) {
        let a: U256 = self.stack_pop();
        let b: U256 = self.stack_pop();

        let result: U256 = a % b;
        self.stack_push(result);
    }

    fn opcode_addmod(&mut self) {
        let a: U256 = self.stack_pop();
        let b: U256 = self.stack_pop();
        let N: U256 = self.stack_pop();

        let result: U256 = (a + b) % N;
        self.stack_push(result);
    }

    fn opcode_mulmod(&mut self) {
        let a: U256 = self.stack_pop();
        let b: U256 = self.stack_pop();
        let N: U256 = self.stack_pop();

        let result: U256 = (a * b) % N;
        self.stack_push(result);
    }

    fn opcode_exp(&mut self) {
        let a: U256 = self.stack_pop();
        let b: U256 = self.stack_pop();

        let mut result: U256 = a.pow(b.as_u32());
        self.stack_push(result);
    }

    fn opcode_signextend(&mut self) {
        println!("0B SIGNEXTEND");
        println!("Not supported!");
    }

    fn opcode_lt(&mut self) {
        let a: U256 = self.stack_pop();
        let b: U256 = self.stack_pop();

        if a < b {
            self.stack_push(U256::new(1));
        } else {
            self.stack_push(U256::new(0));
        }
    }

    fn opcode_gt(&mut self) {
        let a: U256 = self.stack_pop();
        let b: U256 = self.stack_pop();

        if a > b {
            self.stack_push(U256::new(1));
        } else {
            self.stack_push(U256::new(0));
        }
    }

    fn opcode_slt(&mut self) {
        let a: I256 = self.stack_pop().as_i256();
        let b: I256 = self.stack_pop().as_i256();

        if a < b {
            self.stack_push(U256::new(1));
        } else {
            self.stack_push(U256::new(0));
        }
    }

    fn opcode_sgt(&mut self) {
        let a: I256 = self.stack_pop().as_i256();
        let b: I256 = self.stack_pop().as_i256();

        if a > b {
            self.stack_push(U256::new(1));
        } else {
            self.stack_push(U256::new(0));
        }
    }

    fn opcode_eq(&mut self) {
        let a: U256 = self.stack_pop();
        let b: U256 = self.stack_pop();

        if a == b {
            self.stack_push(U256::new(1));
        } else {
            self.stack_push(U256::new(0));
        }
    }

    fn opcode_iszero(&mut self) {
        let a: U256 = self.stack_pop();
        if a == 0 {
            self.stack_push(U256::new(1));
        } else {
            self.stack_push(U256::new(0));
        }
    }

    fn opcode_and(&mut self) {
        let a: U256 = self.stack_pop();
        let b: U256 = self.stack_pop();

        let result: U256 = a & b;
        self.stack_push(result);
    }

    fn opcode_or(&mut self) {
        let a: U256 = self.stack_pop();
        let b: U256 = self.stack_pop();

        let result: U256 = a | b;
        self.stack_push(result);
    }

    fn opcode_xor(&mut self) {
        let a: U256 = self.stack_pop();
        let b: U256 = self.stack_pop();

        let result: U256 = a ^ b;
        self.stack_push(result);
    }

    fn opcode_not(&mut self) {
        let a: U256 = self.stack_pop();
        let result: U256 = !a;
        self.stack_push(result);
    }

    fn opcode_byte(&mut self) {
        let i: U256 = self.stack_pop();
        let x: U256 = self.stack_pop();

        let result = (x >> (31 - i)) & 0xff;
        self.stack_push(result);
    }

    fn opcode_shl(&mut self) {
        let shift: U256 = self.stack_pop();
        let val: U256 = self.stack_pop();

        let result: U256 = val << shift;
        self.stack_push(result);
    }

    fn opcode_shr(&mut self) {
        let shift: U256 = self.stack_pop();
        let val: U256 = self.stack_pop();

        let result: U256 = val >> shift;
        self.stack_push(result);
    }

    fn opcode_sar(&mut self) {
        let shift: U256 = self.stack_pop();
        let val: U256 = self.stack_pop();

        let result: U256 = val >> shift;
        self.stack_push(result);
    }

    fn opcode_sha3(&mut self) {
        let offset: usize = self.stack_pop().as_usize();
        let size: usize = self.stack_pop().as_usize();
        
        let data = self.memory_load(offset, size);
        let mut keccak256 = Keccak256::new();
        keccak256.update(data);

        let result: U256 = U256::from_be_bytes(keccak256.finalize().into());
        self.stack_push(result);
    }

    fn opcode_address(&mut self) {
        let result: U256 = U256::new(0);
        self.stack_push(self.ctx.address);
    }

    fn opcode_balance(&mut self) {
        let address: U256 = self.stack_pop();
        let account: &blockchain::Account = self.blockchain.get_account(address);
        let balance: U256 = account.balance;
        self.stack_push(balance);
    }

    fn opcode_origin(&mut self) {
        self.stack_push(self.ctx.tx.origin);
    }

    fn opcode_caller(&mut self) {
        self.stack_push(self.ctx.msg.from);
    }

    fn opcode_callvalue(&mut self) {
        self.stack_push(self.ctx.msg.value);
    }

    fn opcode_calldataload(&mut self) {
        let offset: usize = self.stack_pop().as_usize();
        let data: [u8; 32] = self.ctx.msg.calldata[offset..offset + 32].try_into().unwrap();
        let result: U256 = U256::from_be_bytes(data);
        self.stack_push(result);
    }

    fn opcode_calldatasize(&mut self) {
        let result: U256 = U256::new(self.ctx.msg.calldata.len().try_into().unwrap());
        self.stack_push(result);
    }

    fn opcode_calldatacopy(&mut self) {
        let dest_offset = self.stack_pop().as_usize();
        let offset = self.stack_pop().as_usize();
        let size = self.stack_pop().as_usize();

        let data = &self.ctx.msg.calldata[offset..offset + size];
        self.memory_store(offset, data.to_vec()); 
    }

    fn opcode_codesize(&mut self) {
        let result: U256 = U256::new(self.ctx.code.len().try_into().unwrap());
        self.stack_push(result);
    }

    fn opcode_codecopy(&mut self) {
        let dest_offset = self.stack_pop().as_usize();
        let offset = self.stack_pop().as_usize();
        let size = self.stack_pop().as_usize();

        let data = &self.ctx.code[offset..offset + size];
        self.memory_store(offset, data.to_vec());
    }

    fn opcode_gasprice(&mut self) {
        let gasprice: U256 = U256::new(self.ctx.tx.gasprice.try_into().unwrap());
        self.stack_push(gasprice);
    }

    fn opcode_extcodesize(&mut self) {
        let address: U256 = self.stack_pop();
        let account: &blockchain::Account = self.blockchain.get_account(address);

        let extcodesize: U256 = U256::new(account.code.len().try_into().unwrap());
        self.stack_push(extcodesize);
    }

    fn opcode_extcodecopy(&mut self) {
        let address: U256 = self.stack_pop();
        let dest_offset: usize = self.stack_pop().as_usize();
        let offset: usize = self.stack_pop().as_usize();
        let size: usize = self.stack_pop().as_usize();

        let account: &blockchain::Account = self.blockchain.get_account(address);

        let data = account.code[offset..offset + size].to_vec();
        self.memory_store(dest_offset, data);
    }

    fn opcode_returndatasize(&mut self) {
        println!("3D RETURNDATASIZE");
    }

    fn opcode_returndatacopy(&mut self) {
        println!("3E RETURNDATACOPY");
    }

    fn opcode_extcodehash(&mut self) {
        let address: U256 = self.stack_pop();
        let account: &blockchain::Account = self.blockchain.get_account(address);

        let mut keccak256 = Keccak256::new();
        keccak256.update(&account.code);
        let value: U256 = U256::from_be_bytes(keccak256.finalize().into());

        self.stack_push(value);
    }

    fn opcode_blockhash(&mut self) {
        let block: blockchain::Block = self.blockchain.get_current_block();
        self.stack_push(block.blockhash);
    }

    fn opcode_coinbase(&mut self) {
        let block: blockchain::Block = self.blockchain.get_current_block();
        self.stack_push(block.coinbase);
    }

    fn opcode_timestamp(&mut self) {
        let block: blockchain::Block = self.blockchain.get_current_block();
        self.stack_push(block.timestamp);
    }

    fn opcode_number(&mut self) {
        let block: blockchain::Block = self.blockchain.get_current_block();
        self.stack_push(block.number);
    }

    fn opcode_difficulty(&mut self) {
        let block: blockchain::Block = self.blockchain.get_current_block();
        self.stack_push(block.difficulty);
    }

    fn opcode_gaslimit(&mut self) {
        let block: blockchain::Block = self.blockchain.get_current_block();
        self.stack_push(block.gaslimit);
    }

    fn opcode_chainid(&mut self) {
        let block: blockchain::Block = self.blockchain.get_current_block();
        self.stack_push(block.chainid);
    }

    fn opcode_selfbalance(&mut self) {
        let account: &blockchain::Account = self.blockchain.get_account(self.ctx.address);
        let balance: U256 = account.balance;
        self.stack_push(balance);
    }

    fn opcode_basefee(&mut self) {
        let block: blockchain::Block = self.blockchain.get_current_block();
        self.stack_push(block.basefee);
    }

    fn opcode_pop(&mut self) {
        self.stack_pop();
    }

    fn opcode_mload(&mut self) {
        let offset: usize = self.stack_pop().as_usize();

        let data: [u8; 32] = self.memory_load(offset, 32).as_slice().try_into().unwrap();
        let value: U256 = U256::from_be_bytes(data);
        self.stack_push(value);
    }

    fn opcode_mstore(&mut self) {
        let offset: usize = self.stack_pop().as_usize();
        let value: [u8; 32] = self.stack_pop().to_be_bytes();

        self.memory_store(offset, value.to_vec());
    }

    fn opcode_mstore8(&mut self) {
        let offset: usize = self.stack_pop().as_usize();
        let value: u8 = self.stack_pop().as_u8();

        self.memory_store(offset, vec![value]);
    }
    
    fn opcode_sload(&mut self) {
        let key: U256 = self.stack_pop();

        let account: &blockchain::Account = self.blockchain.get_account(self.ctx.address);
        let value: U256 = *account.storage.get(&key).unwrap();
        self.stack_push(value);
    }

    fn opcode_sstore(&mut self) {
        let key: U256 = self.stack_pop();
        let value: U256 = self.stack_pop();

        let mut account: &mut blockchain::Account = self.blockchain.get_account(self.ctx.address);
        account.storage.insert(key, value).unwrap();
    }

    fn opcode_jump(&mut self) {
        let offset: usize = self.stack_pop().as_usize();
        self.pc += offset;
    }

    fn opcode_jumpi(&mut self) {
        let counter: usize = self.stack_pop().as_usize();
        let b: U256 = self.stack_pop();
        if b != 0 {
            self.pc += counter;
        }
    }

    fn opcode_pc(&mut self) {
        let pc: U256 = U256::new(self.pc.try_into().unwrap());
        self.stack_push(pc);
    }

    fn opcode_msize(&mut self) {
        let msize: U256 = U256::new(self.memory.len().try_into().unwrap());
        self.stack_push(msize);
    }

    fn opcode_gas(&mut self) {
        let gas: U256 = U256::new(self.ctx.msg.gas.try_into().unwrap());
        self.stack_push(gas);
    }

    fn opcode_jumpdest(&mut self) {}
    
    fn opcode_push0(&mut self) {
        self.stack.push(U256::new(0));
    }

    fn opcode_push(&mut self, length: usize) {
        let hex_value = hex::encode(&self.ctx.code[self.pc .. self.pc + length]);
        let value: U256 = U256::from_str_radix(&hex_value, 16).unwrap();
        self.stack_push(value);
        self.pc += length;
    }

    fn opcode_dup(&mut self, nth: usize) {
        let value: U256 = self.stack[self.stack.len() - nth];
        self.stack_push(value);
    }

    fn opcode_swap(&mut self, nth: usize) {
        let length = self.stack.len();
        self.stack.swap(length - 1, length - 1 - nth);
    }

    fn opcode_log0(&mut self) {}
    fn opcode_log1(&mut self) {}
    fn opcode_log2(&mut self) {}
    fn opcode_log3(&mut self) {}
    fn opcode_log4(&mut self) {}

    fn opcode_create(&mut self) {
        
    }

    fn opcode_call(&mut self) {
        let gas: U256 = self.stack_pop();
        let address: U256 = self.stack_pop();
        let value: U256 = self.stack_pop();
        let argsOffset: usize = self.stack_pop().as_usize();
        let argsSize: usize = self.stack_pop().as_usize();
        let retOffset: U256 = self.stack_pop();
        let retSize: U256 = self.stack_pop();

        let account: &blockchain::Account = self.blockchain.get_account(address);
        let calldata = self.memory[argsOffset..argsOffset + argsSize].to_vec();

        
        let msg = context::MSG::new(
            self.ctx.address,
            address,
            calldata,
            value,
            gas
        );

        let ctx = context::CTX::new(
            self.ctx.tx,
            msg,
            address,
            account.code.clone()
        );

        let mut vm = EVM::new(self.blockchain, ctx);

        let success: bool;
        let returndata: Vec<u8>;
        (success, returndata) = vm.run();
        self.returndata = returndata;
    }

    fn opcode_callcode(&mut self) {
        let gas: U256 = self.stack_pop();
        let address: U256 = self.stack_pop();
        let value: U256 = self.stack_pop();
        let argsOffset: usize = self.stack_pop().as_usize();
        let argsSize: usize = self.stack_pop().as_usize();
        let retOffset: U256 = self.stack_pop();
        let retSize: U256 = self.stack_pop();

        let account: &blockchain::Account = self.blockchain.get_account(address);
        let calldata = self.memory[argsOffset..argsOffset + argsSize].to_vec();

        let msg = context::MSG::new(
            self.ctx.address,
            address,
            calldata,
            value,
            gas
        );

        let ctx = context::CTX::new(
            self.ctx.tx,
            msg,
            self.ctx.address,
            account.code.clone()
        );

        let mut vm = EVM::new(self.blockchain, ctx);
        let returndata: Vec<u8>;
        let success: bool;
        (success, returndata) = vm.run();

        self.returndata = returndata;
    }

    fn opcode_return(&mut self) -> Vec<u8> {
        let offset: usize = self.stack_pop().as_usize();
        let size: usize = self.stack_pop().as_usize();

        let data = self.memory[offset..offset + size].to_vec();
        return data;
    }

    fn opcode_delegatecall(&mut self) {
        let gas: U256 = self.stack_pop();
        let address: U256 = self.stack_pop();
        let value: U256 = self.stack_pop();
        let argsOffset: usize = self.stack_pop().as_usize();
        let argsSize: usize = self.stack_pop().as_usize();
        let retOffset: U256 = self.stack_pop();
        let retSize: U256 = self.stack_pop();

        let account: &blockchain::Account = self.blockchain.get_account(address);
        let calldata = self.memory[argsOffset..argsOffset + argsSize].to_vec();

        let msg = context::MSG::new(
            self.ctx.address,
            address,
            calldata,
            value,
            gas
        );

        let ctx = context::CTX::new(
            self.ctx.tx,
            msg,
            self.ctx.address,
            account.code.clone()
        );

        let mut vm = EVM::new(self.blockchain, ctx);

        let returndata: Vec<u8>;
        let success: bool;
        (success, returndata) = vm.run();

        self.returndata = returndata;
    }

    fn opcode_create2(&mut self) {
    }
    fn opcode_staticcall(&mut self) {
        let gas: U256 = self.stack_pop();
        let address: U256 = self.stack_pop();
        let value: U256 = self.stack_pop();
        let argsOffset: usize = self.stack_pop().as_usize();
        let argsSize: usize = self.stack_pop().as_usize();
        let retOffset: U256 = self.stack_pop();
        let retSize: U256 = self.stack_pop();

        let account: &blockchain::Account = self.blockchain.get_account(address);
        let calldata = self.memory[argsOffset..argsOffset + argsSize].to_vec();

        let msg = context::MSG::new(
            self.ctx.address,
            address,
            calldata,
            value,
            gas
        );

        let ctx = context::CTX::new(
            self.ctx.tx,
            msg,
            address,
            account.code.clone()
        );

        let mut vm = EVM::new(self.blockchain, ctx);
        let returndata = vm.run();

        let returndata: Vec<u8>;
        let success: bool;
        (success, returndata) = vm.run();

        self.returndata = returndata;
    }

    fn opcode_revert(&mut self) -> Vec<u8> {
        let offset: usize = self.stack_pop().as_usize();
        let size: usize = self.stack_pop().as_usize();

        let data = self.memory[offset..offset + size].to_vec();
        return data;
    }

    fn opcode_invalid(&mut self) -> Vec<u8> {
        let data = Vec::new();
        return data;
    }

    fn opcode_selfdestruct(&mut self) {

    }
}
