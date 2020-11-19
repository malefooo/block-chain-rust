use crate::blc::block::{new_block, new_genesis_block, Block};
use std::fmt;
use std::fmt::Write;
use std::fmt::{Display, Formatter};
use crate::blc::proof_of_work::new_proof_of_work;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use num_bigint::BigUint;
use mongodb::options::{InsertOneOptions, FindOneOptions, FindOptions};
use mongodb::bson;
use serde::de::IntoDeserializer;
use mongodb::bson::doc;
use mongodb::error::Error;
use mongodb::results::InsertOneResult;

#[derive(Debug,Deserialize,Serialize, Clone)]
pub struct BlockChain {
    // pub blocks: Vec<Box<Block>>,
    pub tip:String,//最后一个区块的hash，即vec<u8> to hex string
}

impl Display for BlockChain {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "BlockChain(tip:{})", self.tip)
    }
}

impl BlockChain {
    /// 2.0 添加前，查库，如果库里有数据就获取最后一个，如果没有就要求新建区块链
    pub async fn add_block(data: String) -> Option<bool>{
        //获取mongodb collection
        let mongodb_client = super::MONGODB_INSTANCE.get().unwrap();
        let db = mongodb_client.database(super::DATA_BASE);
        let collection = db.collection(super::DATA_COLLECTION);


        let find_cursor = collection.find(None,FindOptions::default()).unwrap();
        let vec_doc : Vec<mongodb::error::Result<bson::Document>> = find_cursor.collect();

        //如果查询到的库为空就返回
        //当然现在数据小可以这么玩，一旦库里数据多了人，这样会相当耗时
        //数据大了的话就查询最后一个就好，最后一个返回的不是非空就好
        return if vec_doc.is_empty() {
            None
        } else {
            let result = vec_doc.last().unwrap();
            let find_doc = result.as_ref().unwrap();

            //解析，反序列化
            let bs = find_doc.get("value").cloned().unwrap();
            let binary = bson::from_bson::<bson::Binary>(bs).unwrap();
            let prev_block = serde_json::from_slice::<Block>(binary.bytes.as_slice()).expect("Deserialize block err");

            //生成新区快
            let mut block = new_block(data,prev_block.hash).await;

            //工作量证明
            let pow = new_proof_of_work(*block).await;
            block = pow.run().await;

            //存新块到库中
            collection.insert_one(block.to_doc().await,InsertOneOptions::default());
            Some(true)
        }
    }

    pub async fn serializer(&self) -> Vec<u8>{
        let block_chain_str = serde_json::to_string(self).expect("block chain to string err.");
        block_chain_str.into_bytes()
    }

    pub async fn print_block_chain(){
        let mongodb_cli = super::MONGODB_INSTANCE.get().unwrap();
        let db = mongodb_cli.database(super::DATA_BASE);
        let collection = db.collection(super::DATA_COLLECTION);

        let find_cursor = collection.find(None,FindOptions::default()).unwrap();
        let vec_doc : Vec<mongodb::error::Result<bson::Document>> = find_cursor.collect();

        if vec_doc.is_empty() {
            println!("the database is empty");
        } else {
            let find_doc = vec_doc.last().unwrap().as_ref().unwrap();

            let mut block_chain = BlockChain{ tip: find_doc.get("key").unwrap().to_string() };

            // 遍历区块链
            let mut iter = block_chain.clone().into_iter();
            loop {
                match iter.next(){
                    None => break,
                    Some(b) => {
                        println!("{}",b);
                        let mut blc = block_chain.clone();
                        blc.tip = BigUint::from_bytes_be(b.prev_block_hash.as_slice()).to_str_radix(16);
                        iter = blc.into_iter();
                    }
                }

            }
        }
    }
}

impl Iterator for BlockChain{
    //饭会的类型是Block
    //在这里使用的是内联？我记得是叫类型内联把，我也记不清了
    //代替了泛型，内置一个返回的type，这个type具体什么类型，自己定义
    //可能我学的还不深，内在的哲学我没get到
    type Item = Block;

    fn next(&mut self) -> Option<Self::Item> {

        //连接mongodb
        let mongodb_cli = super::MONGODB_INSTANCE.get().unwrap();
        let db = mongodb_cli.database(super::DATA_BASE);
        let collection = db.collection(super::DATA_COLLECTION);

        //获取blockchain的hash来从库中找寻block
        let hash = self.tip.as_str();
        let op :Option<bson::Document> = collection.find_one(doc!["key":hash], FindOneOptions::default()).unwrap();

        //找不到直接返回none，迭代结束
        return match op {
            None =>Option::None,
            Some(find_doc)=>{
                //解析bytes
                let bs = find_doc.get("value").cloned().unwrap();
                let binary = bson::from_bson::<bson::Binary>(bs).unwrap();
                let block = serde_json::from_slice::<Block>(binary.bytes.as_slice()).expect("Deserialize block err");
                Option::Some(block)
            }
        }

    }

}

/// 1.0 创建区块链，带有创世区块
/// 2.0 创建创世区块前先看看数据库里有没有数据，没有才给创建，有的话提示只可以添加区块
pub async fn new_block_chain() -> Option<Box<BlockChain>> {
    //创造创世区块
    let block = new_genesis_block().await;

    //获取mongodb，collection
    let mongodb_client = super::MONGODB_INSTANCE.get().unwrap();
    let db = mongodb_client.database(super::DATA_BASE);
    let collection = db.collection(super::DATA_COLLECTION);

    //全查询
    let find_cursor = collection.find(None,FindOptions::default()).unwrap();
    let vec_doc : Vec<mongodb::error::Result<bson::Document>> = find_cursor.collect();

    //如果查询到数据证明库里有数据
    return if vec_doc.is_empty() {
        println!("the database is empty");
        //插入创世区块
        collection.insert_one(block.to_doc().await, InsertOneOptions::default());

        //生成创世区块
        let block_hash = BigUint::from_bytes_be(block.hash.as_slice()).to_str_radix(16);

        let block_chain = BlockChain { tip: block_hash.to_string() };
        Some(Box::new(block_chain))
    } else {
        println!("the database not empty");
        None
    }

}
