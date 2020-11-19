use chrono::{ TimeZone, Utc};
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use tiny_keccak::{Hasher, Sha3};
use crate::blc::proof_of_work::new_proof_of_work;
use num_bigint::BigUint;
use num_traits::{Zero, One};
use crate::utils::leading_zero_to_string;
use serde::{Deserialize, Serialize};
use mongodb::bson;
use mongodb::bson::spec;

#[derive(Debug, Clone,Deserialize,Serialize)]
pub struct Block {
    pub timestamp: i64,
    pub prev_block_hash: Vec<u8>,//还是不能用BigUint，考虑到序列化问题，这个结构没有实现Deserialize,Serialize
    pub data: Vec<u8>,
    pub hash: Vec<u8>,//大数
    pub nonce: u64 //是一个随机值，找到这个随机值，就是循环了几次找到的hash
}

impl Display for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        //格式化打印，因为需要把byte数组换成16进制的字符串
        let prev_hash_str = BigUint::from_bytes_be(self.prev_block_hash.as_slice()).to_str_radix(16);
        // let hash_str = super::u8_vec_to_string(self.hash.clone());
        let hash_str = BigUint::from_bytes_be(self.hash.as_slice()).to_str_radix(16);
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

    pub async fn serializer(&self) -> Vec<u8>{
        let block_chain_str = serde_json::to_string(self).expect("block to string err.");
        block_chain_str.into_bytes()
    }

    pub async fn to_doc(&self) -> bson::Document{
        let mut doc = bson::Document::new();
        let block_serialize = self.serializer().await;
        let block_hash = BigUint::from_bytes_be(self.hash.as_slice()).to_str_radix(16);

        // let b = bson::to_bson(block_serialize.as_slice()).expect("[u8] to bson err");
        let mut b = bson::Binary{ subtype: spec::BinarySubtype::Generic, bytes:  block_serialize };
        doc.insert("key",block_hash);
        doc.insert("value",b);
        doc
    }
}

///创建新的区块
pub async fn new_block(data: String, prev_block_hash: Vec<u8>) -> Box<Block> {
    let mut block = Block {
        timestamp: chrono::Utc::now().timestamp(),

        prev_block_hash,
        data: data.into_bytes(),

        hash: Default::default(),
        nonce: 0
    };

    let pow = new_proof_of_work(block).await;
    let block = pow.run().await;

    block
}

///创建创世区块
pub async fn new_genesis_block() -> Box<Block> {
    let str = "Genesis Block".to_string();

    let block = Block {
        timestamp: chrono::Utc::now().timestamp(),
        prev_block_hash: vec![0],
        data: str.into_bytes(),

        hash: Default::default(),
        nonce: 0
    };


    let pow = new_proof_of_work(block).await;
    let block = pow.run().await;
    block
}
