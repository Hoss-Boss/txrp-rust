use reqwest::blocking::Client;
use serde_json::{json, Value};

pub fn xrp_ledger_call(method: &str, parameters: Value) -> Result<Value, Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = "https://xrplcluster.com/";
    let body = json!({"method": method, "params": [parameters]});

    let respone = client.post(url).json(&body).send()?.error_for_status()?.json::<Value>()?;
    return Ok(respone)
}
