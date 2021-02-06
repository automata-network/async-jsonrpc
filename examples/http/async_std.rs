use async_jsonrpc_client::{BatchTransport, HttpClient, HttpClientError, Response, Transport};

#[async_std::main]
async fn main() -> Result<(), HttpClientError> {
    env_logger::init();

    let client = HttpClient::new("https://rpc.polkadot.io")?;

    let response = client.request("system_chain", None).await?;
    log::info!("Response: {}", Response::Single(response));

    let response = client
        .request_batch(vec![("system_chain", None), ("system_chainType", None)])
        .await?;
    log::info!("Response: {}", Response::Batch(response));

    Ok(())
}
