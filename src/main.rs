mod blc;
mod utils;

fn main() {
    println!("Hello, world!");

    let runtime = tokio::runtime::Runtime::new().unwrap();

    runtime.spawn(async {
        // let block = blc::block::new_block("Hello World!".to_string(),vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]).await;

        //创建创世区块
        // let block = blc::block::new_genesis_block().await;
        // println!("block.prev_block_hash:{:?}",u8_vec_to_string(block.prev_block_hash));
        // println!("block.hash:{:?}",u8_vec_to_string(block.hash));

        //创建区块链
        let mut block_chain = blc::block_chain::new_block_chain().await;
        // println!("{}", block_chain);

        block_chain = block_chain
            .add_block("Send 200$ To A From B".to_string())
            .await;
        block_chain = block_chain
            .add_block("Send 80$ To A From C".to_string())
            .await;
        block_chain = block_chain
            .add_block("Send 290$ To B From C".to_string())
            .await;
        //打印区块链
        // println!("{}", block_chain);

        //便利区块链
        for block in block_chain.blocks {
            println!("{}",block);
        }
    });

}
