use reqwest::blocking::Client;
use serde_json::{json, Value};
use std::sync::OnceLock;

static CLIENT: OnceLock<Client> = OnceLock::new();

pub fn http_client() -> &'static Client {
    CLIENT.get_or_init(Client::new)
}

pub fn xrp_ledger_call(method: &str, parameters: Value) -> Result<Value, Box<dyn std::error::Error>> {
    let url = "https://xrplcluster.com/";
    let body = json!({"method": method, "params": [parameters]});
    let respone = http_client().post(url).json(&body).send()?.error_for_status()?.json::<Value>()?;
    return Ok(respone)
}
