use num_bigint::BigUint;
use std::env;
use crate::blc::block_chain::BlockChain;
use tokio::macros::support::Future;

mod blc;
mod utils;
mod db;



fn main() {

    //初始化数据库
    db::mongodb_con();

    let runtime = tokio::runtime::Runtime::new().unwrap();

    runtime.spawn(async {
        let args: Vec<String> = env::args().collect();
        match args.len() {
            0 => {
                println!("please input:");
                println!("createBlockChain: ./main createBlockChain");
                println!("createBlockChain: ./main printBlockChain");
                println!("addBlock: ./main addBlock \"data\"");
            },
            1 =>{
                match args[0].as_str() {
                    "createBlockChain" => {
                        match blc::block_chain::new_block_chain().await {
                            None => {println!("the block chain is exist!")}
                            Some(blc) => {println!("create block chain success")}
                        }

                    }
                    "printBlockChain" => {
                        BlockChain::print_block_chain();
                    }
                    &_ => {}
                }
            },
            2 =>{
                match args[0].as_str() {
                    "addBlock" => {
                        let data = &args[1];

                        match BlockChain::add_block(data.to_string()).await{
                            None => println!("the database not exist"),
                            Some(_) => println!("add block to chain true"),
                            _ => {}
                        }
                    }
                    &_ => {}
                }
            },
            _ => {}
        }

    });

}
