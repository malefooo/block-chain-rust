
use mongodb::sync::Client;
use once_cell::sync::OnceCell;
use mongodb::options::ClientOptions;

const MONGODB_ADDR:&str = "mongodb://47.112.194.50:27017";
pub const DATA_BASE:&str = "block_chain_db";
pub const DATA_COLLECTION:&str = "block";

pub static MONGODB_INSTANCE: OnceCell<Client> = OnceCell::new();

pub fn mongodb_con(){
    let client = Client::with_uri_str(MONGODB_ADDR).unwrap();
    MONGODB_INSTANCE.set(client);
}

