use ethnum::{U256};
use std::collections::HashMap;


#[derive(Copy, Clone)]
pub struct Block {
    pub blockhash: U256,
    pub coinbase: U256,
    pub timestamp: U256,
    pub number: U256,
    pub difficulty: U256,
    pub gaslimit: U256,
    pub chainid: U256,
    pub basefee: U256
}

pub struct Account {
    pub nonce: U256,
    pub balance: U256,
    pub storage: HashMap<U256, U256>,
    pub code: Vec<u8>
}


pub struct BlockChain {
    blocks: Vec<Block>,
    accounts: HashMap<U256, Account>
}

impl BlockChain {
    pub fn new() -> BlockChain {
        return BlockChain {
            blocks: Vec::new(),
            accounts: HashMap::new()
        };
    }

    pub fn add_contract(&mut self, address: &str, bytecode: Vec<u8>) {
        let account = Account {
            nonce: U256::new(0),
            balance: U256::new(0),
            storage: HashMap::new(),
            code: bytecode
        };
        self.accounts.insert(
            U256::from_str_hex(address).unwrap(),
            account
        );
    }

    pub fn get_account(&mut self, address: U256) -> &mut Account {
        let mut account = self.accounts.get_mut(&address).unwrap();
        return account;
    }

    pub fn get_current_block(&self) -> Block {
        return self.blocks[self.blocks.len() - 1];
    }
}
