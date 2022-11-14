use elasticsearch::{http::{transport::{TransportBuilder, SingleNodeConnectionPool},Url}, auth::Credentials, cert::CertificateValidation};
use elasticsearch::Elasticsearch;
use elasticsearch::IndexParts;
use serde_json::json;
use std::fs::File;
use std::io::prelude::*;

mod prize_model;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = Url::parse("https://192.168.107.193:9200")?;
    let conn_pool = SingleNodeConnectionPool::new(url);
    let credentials = Credentials::Basic("elastic".into(), "QNiMLXho6f9+V66WRZYz".into());
    let transport = TransportBuilder::new(conn_pool)
        .disable_proxy()
        .auth(credentials)
        .cert_validation(CertificateValidation::None)
        .build()?;
    let client = Elasticsearch::new(transport);

    // Parse json data file
    let mut file = File::open("/home/boccard/cluster/data/nobel-prize.json")?;
    let mut data_file = String::new();
    file.read_to_string(&mut data_file)?;
    let prizes: Vec<prize_model::Prize> = serde_json::from_str(&data_file).expect("JSON was not well-formatted");

    for (i, prize) in prizes.iter().enumerate() {
        let response = client
            .index(IndexParts::IndexId("prizes", i.to_string().as_str()))
            .body(json!(prize))
            .send()
            .await?;

    let _successful = response.status_code().is_success();
    }
    
    Ok(())
}
