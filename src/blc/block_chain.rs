use crate::blc::block::{new_block, new_genesis_block, Block};
use std::fmt;
use std::fmt::Write;
use std::fmt::{Display, Formatter};
use crate::blc::proof_of_work::new_proof_of_work;

#[derive(Debug)]
pub struct BlockChain {
    pub blocks: Vec<Box<Block>>,
}

impl Display for BlockChain {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut vec_block_str = String::new();
        for block in self.blocks.clone() {
            write!(vec_block_str, "{}", block).expect("---ERROR---");
        }
        write!(f, "BlockChain(blocks:{})", vec_block_str)
    }
}

impl BlockChain {
    pub async fn add_block(mut self, data: String) -> Box<BlockChain> {
        //获取上一个区块的hash
        let prev_block = self.blocks.last().unwrap();
        let mut block = new_block(data, prev_block.hash.clone()).await;

        // block = block.set_hash().await;
        let pow = new_proof_of_work(*block).await;
        let box_block = pow.run().await;
        self.blocks.push(box_block);
        Box::new(self)
    }
}

///创建区块链，带有创世区块
pub async fn new_block_chain() -> Box<BlockChain> {
    let block = new_genesis_block().await;

    let block_chain = BlockChain {
        blocks: vec![block],
    };

    Box::new(block_chain)
}
