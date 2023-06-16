use std::io::{self,Read};
use hex;
use ethnum::{U256};

mod context;
mod evm;
mod blockchain;


fn main() {
    /*
    let mut buffer = Vec::new();
    let mut handle = io::stdin().lock();
    handle.read_to_end(&mut buffer);
    println!("{:?}", buffer);
    */

    let inp = "604260005260206000F3";
    let bytecode: Vec<u8> = hex::decode(inp).unwrap();

    let alice = "0x41414141";
    let contract_address = "0x42424242";

    let mut blockchain = blockchain::BlockChain::new();
    blockchain.add_contract(contract_address, bytecode.clone());

    let origin: U256 = U256::from_str_hex(alice).unwrap();
    let gasprice: U256 = U256::new(1000);
    let from: U256 = U256::from_str_hex(alice).unwrap();
    let to: U256 = U256::from_str_hex(contract_address).unwrap();
    let calldata: Vec<u8> =  Vec::new();
    let value: U256 = U256::new(0);
    let gas: U256 = U256::new(1000);
    let address: U256 = U256::from_str_hex(contract_address).unwrap();

    let mut tx = context::TX::new(origin, gasprice);
    let mut msg = context::MSG::new(from, to, calldata, value, gas);

    let mut ctx = context::CTX::new(
        tx,
        msg,
        address,
        bytecode.clone()
    );

    let mut vm = evm::EVM::new(&mut blockchain, ctx);
    vm.run();
    vm.print_stack();
    vm.print_memory();
}
