use ethnum::{U256};


pub fn bool_to_u256(value: bool) -> U256 {
    if value {
        return U256::new(1);
    } else {
        return U256::new(0);
    }
}


pub fn encode_rlp(address: &[u8], nonce: U256) -> Vec<u8> {
    let mut data: Vec<u8> = Vec::new();
    if nonce == 0 {
        data.extend([0xD6, 0x94]);
        data.extend(address);
        data.push(0x80); 
    }
    else if nonce <= 0x7f {
        data.extend([0xD6, 0x94]);
        data.extend(address);
        data.push(nonce.as_u8());
    }
    else if nonce <= 0xff {
        data.extend([0xD7, 0x94]);
        data.extend(address);
        data.push(0x81);
        data.push(nonce.as_u8());
    }
    else if nonce <= 0xffff {
        data.extend([0xD8, 0x94]);
        data.extend(address);
        data.push(0x82);
        data.extend(nonce.as_u16().to_be_bytes());
    }
    else if nonce <= 0xffffff {
        data.extend([0xD9, 0x94]);
        data.extend(address);
        data.push(0x83);
        data.extend(nonce.as_u32().to_be_bytes());
    }
    else {
        data.extend([0xDA, 0x94]);
        data.extend(address);
        data.push(0x84);
        data.extend(nonce.as_u32().to_be_bytes());
    }

    return data;
}
