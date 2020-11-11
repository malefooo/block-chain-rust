mod BLC;

fn main() {
    println!("Hello, world!");

    let runtime = tokio::runtime::Runtime::new().unwrap();



    runtime.spawn(async {
        let block = BLC::block::new_block("Hello World!".to_string(),vec![]).await;
        println!("block:{:?}",block);
    });
}
