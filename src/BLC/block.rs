use std::io::Bytes;
use std::fmt::Debug;

#[derive(Debug,Clone)]
pub struct Block{
    pub timestamp:i64,
    pub prev_block_hash:Vec<u8>,
    pub data:Vec<u8>,
    pub hash:Vec<u8>,
}

impl Block {
    pub fn set_hash(mut self) -> Block{
        let time_str = self.timestamp.to_string();
        let time_bytes = time_str.into_bytes();

        let mut all_bytes:Vec<u8> = Vec::new();
        all_bytes.extend(time_bytes.iter());
        all_bytes.extend(self.data.iter());
        all_bytes.extend(self.prev_block_hash.iter());


        self
    }
}

///创建新的区块
pub async fn new_block(data:String, prevBlockHash:Vec<u8>) -> Block{
    let block = Block{
        timestamp: chrono::Local::now().timestamp_millis(),
        prev_block_hash: prevBlockHash,
        data: data.into_bytes(),
        hash: vec![]
    };

    let block = block.set_hash();

    block

}