use clickhouse::Client;
use clickhouse::Row;
use serde::{Deserialize, Serialize};

#[derive(Debug, Row, Serialize, Deserialize)]
struct Database {
    name: String,
}

pub async fn ck_http() {
    let client = Client::default()
    .with_url("http://service-terrabase-fu24nvf8lr.terrabase-fu24nvf8lr-hb-public.jvessel2.jdcloud.com:8123")
    .with_user("sample")
    .with_password("Git785230");

    let sql = "SHOW databases";
    let r = client.query(sql).fetch_all::<Database>().await;
    println!("result is: {:?}", r);
}
