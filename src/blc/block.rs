use chrono::{ TimeZone, Utc};
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use tiny_keccak::{Hasher, Sha3};
use crate::blc::proof_of_work::new_proof_of_work;
use num_bigint::BigUint;
use num_traits::{Zero, One};
use crate::utils::leading_zero_to_string;

#[derive(Debug, Clone)]
pub struct Block {
    pub timestamp: i64,
    pub prev_block_hash: BigUint,
    pub data: Vec<u8>,
    pub hash: BigUint,//大数
    pub nonce: u64 //是一个随机值，找到这个随机值，就是循环了几次找到的hash
}

impl Display for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        //格式化打印，因为需要把byte数组换成16进制的字符串
        let prev_hash_str = self.prev_block_hash.to_str_radix(16);
        // let hash_str = super::u8_vec_to_string(self.hash.clone());
        let hash_str = self.hash.to_str_radix(16);
        //时间格式化
        let dt = Utc.timestamp(self.timestamp, 0);

        write!(
            f,
            " Block (timestamp:{}, prev_block_hash:{}, data:{}, hash:{}, nonce:{}) ",
            dt.to_string(),
            leading_zero_to_string(prev_hash_str,64),
            String::from_utf8(self.data.clone()).unwrap(),
            leading_zero_to_string(hash_str,64),
            self.nonce
        )
    }
}



impl Block {
    // 设置hash，这个为一个接受方法，因为会消耗自身
    // demo，会弃置
    // pub async fn set_hash(mut self) -> Box<Block> {
    //     let time_str = self.timestamp.to_string();
    //     let time_bytes = time_str.into_bytes();
    //
    //     //将所有数据合并
    //     let mut all_bytes: Vec<u8> = Vec::new();
    //     all_bytes.extend(time_bytes.iter());
    //     all_bytes.extend(self.data.iter());
    //     all_bytes.extend(self.prev_block_hash.iter());
    //
    //     //hash算法
    //     let mut sha3 = Sha3::v256();
    //     let mut output = [0u8; 32];
    //     sha3.update(all_bytes.as_slice());
    //     sha3.finalize(&mut output);
    //
    //     self.hash = output.to_vec();
    //     Box::new(self)
    // }
}

///创建新的区块
pub async fn new_block(data: String, prev_block_hash: BigUint) -> Box<Block> {
    let mut block = Block {
        timestamp: chrono::Utc::now().timestamp(),

        prev_block_hash,
        data: data.into_bytes(),

        hash: Default::default(),
        nonce: 0
    };

    // 最早版本的生成hash，弃之
    // block = block.set_hash().await;
    let pow = new_proof_of_work(block).await;
    let block = pow.run().await;

    block
}

///创建创世区块
pub async fn new_genesis_block() -> Box<Block> {
    let str = "Genesis Block".to_string();

    let block = Block {
        timestamp: chrono::Utc::now().timestamp(),
        prev_block_hash: BigUint::zero(),
        data: str.into_bytes(),

        hash: Default::default(),
        nonce: 0
    };

    // let block = block.set_hash().await;

    let pow = new_proof_of_work(block).await;
    let block = pow.run().await;
    block
}
