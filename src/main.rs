use num_bigint::BigUint;
use std::env;
use crate::blc::block_chain::BlockChain;
use tokio::macros::support::Future;
use clap::{Arg, App};
use once_cell::sync::OnceCell;

mod blc;
mod utils;
mod db;

static ARGS_INSTANCE:OnceCell<Vec<String>> = OnceCell::new();

fn main() {
    let args: Vec<String> = env::args().collect();
    ARGS_INSTANCE.set(args);
    //初始化数据库
    db::mongodb_con();

    let runtime = tokio::runtime::Runtime::new().unwrap();

    runtime.spawn(async {
        let args = ARGS_INSTANCE.get().unwrap();
        match args.len() {
            1 =>{
                let mut var = String::new();
                if cfg!(target_os = "windows"){
                    var = "block-chain-rust.exe".to_string();
                } else if cfg!(target_os = "linux"){
                    var = "./block-chain-rust".to_string();
                }
                println!("please input:");
                println!("createBlockChain: {} createBlockChain",var);
                println!("createBlockChain: {} printBlockChain",var);
                println!("addBlock: {} addBlock \"data\"",var);
            },
            2 =>{
                match args[1].as_str() {
                    "createBlockChain" => {
                        match blc::block_chain::new_block_chain().await {
                            None => {println!("the block chain is exist!")}
                            Some(blc) => {println!("create block chain success")}
                        }

                    }
                    "printBlockChain" => {
                        BlockChain::print_block_chain().await;
                    }
                    &_ => {}
                }
            },
            3 =>{
                match args[1].as_str() {
                    "addBlock" => {
                        let data = &args[2];

                        match BlockChain::add_block(data.to_string()).await{
                            None => println!("the database not exist"),
                            Some(_) => println!("add block to chain true"),
                            _ => {}
                        }
                    }
                    &_ => {}
                }
            }
            _ => {}
        }

    });

}
