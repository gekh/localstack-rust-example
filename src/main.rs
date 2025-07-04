use lambda_runtime::{LambdaEvent, service_fn};
use serde_json::{Value, json};

#[tokio::main]
async fn main() -> Result<(), lambda_runtime::Error> {
    let func = service_fn(handler);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn handler(event: LambdaEvent<Value>) -> Result<Value, lambda_runtime::Error> {
    println!("Received event: {:?}", event);
    Ok(json!({
        "statusCode": 200,
        "body": "Hello from Rust Lambda"
    }))
}
