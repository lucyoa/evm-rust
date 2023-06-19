use std::io::{self,Read};
use hex;
use ethnum::{U256};
use colored::Colorize;


mod context;
mod evm;
mod blockchain;
mod utils;
mod instructions;


fn main() {
    /*
    let mut buffer = Vec::new();
    let mut handle = io::stdin().lock();
    handle.read_to_end(&mut buffer);
    println!("{:?}", buffer);
    */

    // let inp = "604260005260206000F3";
    // let inp = "3456FDFDFDFDFDFD5B00"; // evm puzzles 1
    // let inp = "34380356FDFD5B00FDFD"; // evm puzzles 2
    // let inp = "3656FDFD5B00"; // evm puzzles 3
    // let inp = "34381856FDFDFDFDFDFD5B00"; // evm puzzles 4
    // let inp = "34800261010014600C57FDFD5B00FDFD"; // evm puzzles 5
    // let inp = "60003556FDFDFDFDFDFD5B00"; // evm puzzles 6
    // let inp = "36600080373660006000F03B600114601357FD5B00"; // evm puzzles 7
    // let inp = "36600080373660006000F0600080808080945AF1600014601B57FD5B00"; // evm puzzles 8
    // let inp = "36600310600957FDFD5B343602600814601457FD5B00"; // evm puzzles 9
    let inp = "38349011600857FD5B3661000390061534600A0157FDFDFDFD5B00"; // evm puzzles 10
    let bytecode: Vec<u8> = hex::decode(inp).unwrap();

    let alice = "0x41414141";
    let contract_address = "0x42424242";

    let mut blockchain = blockchain::BlockChain::new();
    blockchain.add_contract(contract_address, bytecode.clone());

    let origin: U256 = U256::from_str_hex(alice).unwrap();
    let gasprice: U256 = U256::new(1000);
    let from: U256 = U256::from_str_hex(alice).unwrap();
    let to: U256 = U256::from_str_hex(contract_address).unwrap();
    let calldata: Vec<u8> = hex::decode("FFFFFF").unwrap();
    let value: U256 = U256::new(15);
    let gas: U256 = U256::new(10000000);
    let address: U256 = U256::from_str_hex(contract_address).unwrap();

    let mut tx = context::TX::new(origin, gasprice);
    let mut msg = context::MSG::new(from, to, calldata, value, gas);

    let mut ctx = context::CTX::new(
        tx,
        msg,
        address,
        bytecode.clone()
    );

    let mut success: bool = false;
    let mut returndata: Vec<u8>;
    let mut vm = evm::EVM::new(&mut blockchain, ctx);
    (success, returndata) = vm.run();
    vm.print_stack();
    vm.print_memory();

    blockchain.clean_contracts();

    println!("--- Return Data ---");
    println!(
        "0x{}\n{:?}",
        hex::encode(returndata.clone()),
        String::from_utf8(returndata).unwrap()
    );
    println!("-------------------");

    if success {
        println!("{}", "success".green());
    } else { 
        println!("{}", "revert".red());
    }
}
