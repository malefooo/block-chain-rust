use crate::blc::block::Block;
use num_bigint::{Sign, BigUint, BigInt};
use tiny_keccak::{Sha3, Hasher};
use num_traits::cast::ToPrimitive;
use num_traits::{Zero, One};
use crate::utils::leading_zero_to_string;

pub struct ProofOfWork{
    pub block:Box<Block>,//要进行工作的区块
    pub target:BigUint,//目标值
}

///难度值
const target_bits:u64 = 8;

///计算范围
const max_nonce:u64 = std::u64::MAX;

impl ProofOfWork {
    pub async fn run(mut self)->Box<Block>{
        let mut nonce = 0u64;
        loop {
            if nonce > max_nonce {
                break;
            }
            self.block.nonce = nonce;
            if self.calculation_hash().await{
                break;
            } else {
                nonce = nonce+1;
            }
        }

        Box::new(*self.block)
    }

    async fn calculation_hash(&mut self) -> bool{

        let all_u8_vec = self.prepare_data().await;

        let mut sha3 = Sha3::v256();
        let mut output = [0u8; 32];
        sha3.update(all_u8_vec.as_slice());
        sha3.finalize(&mut output);

        let hash_bigint = num_bigint::BigUint::from_bytes_be(&output);
        // println!("hash:{number:>0width$},nonce:{val}",number=hash_bigint.to_str_radix(16),width=64,val=self.block.nonce);

        return if hash_bigint < self.target {
            self.block.hash = output.to_vec();
            true
        } else {
            false
        }
    }

    async fn prepare_data(&self) -> Vec<u8>{
        let time_str = self.block.timestamp.to_string();
        let time_bytes = time_str.into_bytes();
        let mut all_u8_vec:Vec<u8> = Vec::new();
        all_u8_vec.extend(self.block.data.iter());
        all_u8_vec.extend(self.block.hash.iter());
        all_u8_vec.extend(self.block.prev_block_hash.iter());
        all_u8_vec.extend(self.block.nonce.to_be_bytes().iter());
        all_u8_vec.extend(time_bytes.iter());
        all_u8_vec
    }
}

pub async fn new_proof_of_work(block:Block) -> Box<ProofOfWork>{

    let mut target = BigUint::one();
    //在实际计算的时候，16进制表示时，一个位=8，当target_bits=8时，实则移了1位，
    //0100000000000000000000000000000000000000000000000000000000000000
    target = target << (256-target_bits) as usize;

    let pow = ProofOfWork{
        block: Box::new(block),
        target
    };
    Box::new(pow)
}

#[cfg(test)]
mod proof_of_work_mod {
    use crate::blc::proof_of_work::new_proof_of_work;
    use crate::blc::block::Block;

    #[test]
    fn test(){
        new_proof_of_work(Block{
            timestamp: 0,
            prev_block_hash: vec![],
            data: vec![],
            hash: vec![],
            nonce: 0
        });
    }
}