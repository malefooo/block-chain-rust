
use std::fmt::{Debug, Display, Formatter};
use tiny_keccak::{Sha3, Hasher};
use std::fmt;
use chrono::{DateTime, TimeZone, Utc};

#[derive(Debug,Clone)]
pub struct Block{
    pub timestamp:i64,
    pub prev_block_hash:Vec<u8>,
    pub data:Vec<u8>,
    pub hash:Vec<u8>,
}

impl Display for Block{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {

        let prev_hash_str = super::u8_vec_to_string(self.prev_block_hash.clone());
        let hash_str = super::u8_vec_to_string(self.hash.clone());
        let dt = Utc.timestamp(self.timestamp,0);
        write!(f," Block (timestamp:{}, prev_block_hash:{}, data:{}, hash:{}) ",
               dt.to_string(),
               prev_hash_str,
               String::from_utf8(self.data.clone()).unwrap(),
               hash_str)
    }
}

impl Block {
    pub async fn set_hash(mut self) -> Box<Block>{
        let time_str = self.timestamp.to_string();
        let time_bytes = time_str.into_bytes();

        let mut all_bytes:Vec<u8> = Vec::new();
        all_bytes.extend(time_bytes.iter());
        all_bytes.extend(self.data.iter());
        all_bytes.extend(self.prev_block_hash.iter());

        let mut sha3 = Sha3::v256();
        // let mut output:Vec<u8> = Vec::new();
        let mut output = [0u8; 32];
        sha3.update(all_bytes.as_slice());
        sha3.finalize(&mut output);
        self.hash = output.to_vec();
        Box::new(self)
    }
}

///创建新的区块
pub async fn new_block(data:String, prev_block_hash:Vec<u8>) -> Box<Block>{
    let mut block = Box::new(Block{
        timestamp: chrono::Utc::now().timestamp(),
        prev_block_hash,
        data: data.into_bytes(),
        hash: vec![]
    });

    block = block.set_hash().await;

    block
}

///创建创世区块
pub async fn new_genesis_block() -> Box<Block>{

    let str = "Genesis Block".to_string();

    let block =  Block{
        timestamp: chrono::Utc::now().timestamp(),
        prev_block_hash: vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        data: str.into_bytes(),
        hash: vec![]
    };

    let block = block.set_hash().await;

    block
}