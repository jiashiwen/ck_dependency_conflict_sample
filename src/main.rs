// use clickhouse_rs::Pool;

// use clickhouse::Client;
// use clickhouse::Row;
// use serde::{Deserialize, Serialize};

// #[derive(Debug, Row, Serialize, Deserialize)]
// struct Database {
//     name: String,
// }

use ck_http;
use ck_tcp;

#[tokio::main]
async fn main() {
    println!("main");
    ck_tcp::ck_tcp().await;
    ck_http::ck_http().await;
}

// async fn ck_tcp() {
//     let database_url="tcp://sample:Git785230@service-terrabase-fu24nvf8lr.terrabase-fu24nvf8lr-hb-public.jvessel2.jdcloud.com:9000?compression=lz4".to_string();
//     let pool = Pool::new(database_url);

//     let mut client = pool.get_handle().await.unwrap();
//     let sql = "show databases;";
//     let r = client.query(sql).fetch_all().await;
//     println!("result is: {:?}", r);
// }

// async fn ck_http() {
//     let client = Client::default()
//     .with_url("http://service-terrabase-fu24nvf8lr.terrabase-fu24nvf8lr-hb-public.jvessel2.jdcloud.com:8123")
//     .with_user("sample")
//     .with_password("Git785230");

//     let sql = "SHOW databases";
//     let r = client.query(sql).fetch_all::<Database>().await;
//     println!("result is: {:?}", r);
// }
