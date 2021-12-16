extern crate tokio;
extern crate kudu;
extern crate krpc;
extern crate futures;

use std::{env, error};
use std::fmt::Debug;
use std::path::PathBuf;
use std::sync::Arc;
use futures::{Future, Stream};
use tokio::runtime::Runtime;
use kudu::{Client, Error, Options};

fn get_env(udf_env: &str) -> String {
    // Try user defined env.
    if let Ok(dir) = env::var(udf_env) {
        let v=dir.clone();
        v
    }else{
        panic!(format!(
            "env `{}` is undefined or has empty value\n\
             HINT: try running `export master_address=127.0.0.1:7051`",
            udf_env,
        ))
    }
}


fn list_tables() {
    let mut runtime = Runtime::new().unwrap();
    let mut client = runtime.block_on( Client::new(vec![get_env("master_address")], Options::default())).unwrap();
    let tables = runtime.block_on( client.tables()).unwrap();
    for table in tables{
        println!("{}",table.0)
    }
}

fn list_column(){
    let mut runtime = Runtime::new().unwrap();
    let mut client = runtime.block_on( Client::new(vec![get_env("master_address")], Options::default())).unwrap();
    let table =runtime.block_on( client.open_table(get_env("table_name"))).unwrap();
    let schema = table.schema();
    for col in schema.columns(){
        println!("{:?}",col);

    }
}

fn select(){
    let mut runtime = Runtime::new().unwrap();
    let mut client = runtime.block_on( Client::new(vec![get_env("master_address")], Options::default())).unwrap();
    let table =runtime.block_on( client.open_table(get_env("table_name"))).unwrap();
    let scan = table.scan_builder().select(vec!["appid","ds"]).unwrap().build();

    let batches = runtime
        .block_on(::futures::future::lazy(|| scan.collect()))
        .unwrap();

    for batch in batches {
        for row in batch.into_iter() {
            println!("{:?}",row)
        }
    }
}

fn main() {
    let _ = env_logger::init();
    list_tables();
    list_column();
    select();
}