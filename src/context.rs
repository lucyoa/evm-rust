use ethnum::{U256};

#[derive(Clone, Copy)]
pub struct TX {
    pub origin: U256,
    pub gasprice: U256,
}

impl TX {
    pub fn new(origin: U256, gasprice: U256) -> TX{
        return TX {
            origin: origin,
            gasprice: gasprice
        };
    }
}

pub struct MSG {
    pub from: U256,
    pub to: U256,
    pub calldata: Vec<u8>,
    pub value: U256,
    pub gas: U256,
}
impl MSG {
    pub fn new(from: U256, to: U256, calldata: Vec<u8>, value: U256, gas: U256) -> MSG {
        return MSG {
            from: from,
            to: to,
            calldata: calldata,
            value: value,
            gas: gas
        };
    }
}

pub struct CTX {
    pub tx: TX,
    pub msg: MSG,
    pub address: U256,
    pub code: Vec<u8>
}

impl CTX {
    pub fn new(tx: TX, msg: MSG, address: U256, code: Vec<u8>) -> CTX {
        return CTX {
            tx: tx,
            msg: msg,
            address: address,
            code: code
        };
    }
    /*
    pub fn new(origin: &str, from: &str, to: &str, value: U256, calldata: Vec<u8>, gas: U256, gasprice: U256, address: U256, code: Vec<u8>) -> CTX {
        return CTX {
            tx: TX {
                origin: U256::from_str_hex(origin).unwrap(),
                gasprice: gasprice,
            },
            msg: MSG {
                from: U256::from_str_hex(from).unwrap(),
                to: U256::from_str_hex(to).unwrap(),
                value: value,
                calldata: calldata,
                gas: gas,
            },
            address: address,
            code: code
        };
    }
    */
}
