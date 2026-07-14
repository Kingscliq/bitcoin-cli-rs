use anyhow::{Result, bail};
use serde_json::Value;

use crate::rpc::BitcoinRpcClient;

pub(super) fn run(client: &BitcoinRpcClient, method: String, params: Vec<String>) -> Result<()> {
    if method.trim().is_empty() {
        bail!("RPC method cannot be empty");
    }

    tracing::info!(
        command = "rpc",
        rpc_method = method,
        "executing CLI command"
    );

    let params = Value::Array(parse_params(params));
    let result: Value = client.call(&method, params)?;
    println!("{}", serde_json::to_string_pretty(&result)?);

    Ok(())
}

fn parse_params(params: Vec<String>) -> Vec<Value> {
    params
        .into_iter()
        .map(|param| serde_json::from_str::<Value>(&param).unwrap_or(Value::String(param)))
        .collect()
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn parses_json_values_and_preserves_plain_strings() {
        let params = parse_params(vec![
            "200".to_owned(),
            "true".to_owned(),
            "null".to_owned(),
            "block-hash".to_owned(),
            "[1,2]".to_owned(),
            r#"{"verbosity":2}"#.to_owned(),
        ]);

        assert_eq!(
            params,
            vec![
                json!(200),
                json!(true),
                Value::Null,
                json!("block-hash"),
                json!([1, 2]),
                json!({ "verbosity": 2 }),
            ]
        );
    }
}
