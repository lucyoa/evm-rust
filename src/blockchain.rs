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
    pub basefee: U256,
}

pub struct Account {
    pub nonce: U256,
    pub balance: U256,
    pub storage: HashMap<U256, U256>,
    pub code: Vec<u8>
}

struct Log {
    pub data: Vec<u8>,
    pub topic1: U256,
    pub topic2: U256,
    pub topic3: U256,
    pub topic4: U256
}


pub struct BlockChain {
    blocks: Vec<Block>,
    accounts: HashMap<U256, Account>,
    logs: Vec<Log>,
    destroy_list: Vec<U256>
}

impl BlockChain {
    pub fn new() -> BlockChain {
        return BlockChain {
            blocks: Vec::new(),
            accounts: HashMap::new(),
            logs: Vec::new(),
            destroy_list: Vec::new()
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

    pub fn add_contract_bin(&mut self, address: U256, bytecode: Vec<u8>) {
        let account = Account {
            nonce: U256::new(0),
            balance: U256::new(0),
            storage: HashMap::new(),
            code: bytecode
        };
        self.accounts.insert(
            address,
            account
        );
    }

    pub fn register_to_destroy(&mut self, address: U256, recipient: U256) {
        let mut contract_account = self.accounts.get_mut(&address).unwrap();
        let mut balance = contract_account.balance;
        contract_account.balance = U256::new(0);

        let mut recipient_account = self.accounts.get_mut(&recipient).unwrap();
        recipient_account.balance = balance;
        self.destroy_list.push(address);
    }

    pub fn clean_contracts(&mut self) {
        for contract in &self.destroy_list {
            self.accounts.remove(contract);
        }
    }

    pub fn get_account(&mut self, address: U256) -> &mut Account {
        let mut account = self.accounts.get_mut(&address).unwrap();
        return account;
    }

    pub fn get_current_block(&self) -> Block {
        return self.blocks[self.blocks.len() - 1];
    }

    pub fn add_log0(&mut self, data: Vec<u8>) {
        let log: Log = Log{
            data: data,
            topic1: U256::new(0),
            topic2: U256::new(0),
            topic3: U256::new(0),
            topic4: U256::new(0)
        };
        self.logs.push(log);
    }

    pub fn add_log1(&mut self, data: Vec<u8>, topic1: U256) {
        let log: Log = Log{
            data: data,
            topic1: topic1,
            topic2: U256::new(0),
            topic3: U256::new(0),
            topic4: U256::new(0)
        };
        self.logs.push(log);
    }

    pub fn add_log2(&mut self, data: Vec<u8>, topic1: U256, topic2: U256) {
        let log: Log = Log{
            data: data,
            topic1: topic1,
            topic2: topic2,
            topic3: U256::new(0),
            topic4: U256::new(0)
        };
        self.logs.push(log);
    }

    pub fn add_log3(&mut self, data: Vec<u8>, topic1: U256, topic2: U256, topic3: U256) {
        let log: Log = Log{
            data: data,
            topic1: topic1,
            topic2: topic2,
            topic3: topic3,
            topic4: U256::new(0)
        };
        self.logs.push(log);
    }

    pub fn add_log4(&mut self, data: Vec<u8>, topic1: U256, topic2: U256, topic3: U256, topic4: U256) {
        let log: Log = Log{
            data: data,
            topic1: topic1,
            topic2: topic2,
            topic3: topic3,
            topic4: topic4
        };
        self.logs.push(log);
    }
}
