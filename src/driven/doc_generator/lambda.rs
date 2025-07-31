// driven/doc_generator/aws.rs
use super::{DocumentGenerator, DocGenError};
use async_trait::async_trait;

// 1) Bring in the new defaults loader (instead of load_from_env())
use aws_config::{defaults, meta::region::RegionProviderChain};
use aws_sdk_lambda::{Client as LambdaClient, types::InvocationType, config::BehaviorVersion};
use aws_smithy_types::Blob;                // for wrapping the payload
use serde_json::Value;

pub struct AwsLambdaDocGenerator {
    client: LambdaClient,
    function_name: String,
}

impl AwsLambdaDocGenerator {
    pub async fn new(function_name: impl Into<String>) -> Self {
        // we still pick up the region/provider from the environment/provider chain
        let region_provider = RegionProviderChain::default_provider();

        // 2) Use aws_config::defaults() to silence the deprecation warning
        let behaviour_version = BehaviorVersion::latest();
        let config = defaults(behaviour_version)
            .region(region_provider)
            .load()
            .await;

        let client = LambdaClient::new(&config);
        AwsLambdaDocGenerator {
            client,
            function_name: function_name.into(),
        }
    }
}

#[async_trait]
impl DocumentGenerator for AwsLambdaDocGenerator {
    async fn generate(&self, instructions: &Value) -> Result<String, DocGenError> {
        // 3) Serialize your JSON into bytes
        let bytes = serde_json::to_vec(instructions)
            .map_err(|e| DocGenError::InvocationError(e.to_string()))?;

        // 4) Invoke with .invocation_type(...) and wrap the bytes in a Blob
        let response = self.client
            .invoke()
            .function_name(&self.function_name)
            .invocation_type(InvocationType::RequestResponse)   // synchronous call :contentReference[oaicite:1]{index=1}
            .payload(Blob::new(bytes))                         // Blob from aws-smithy-types :contentReference[oaicite:2]{index=2}
            .send()
            .await
            .map_err(|e| DocGenError::InvocationError(e.to_string()))?;

        // 5) Pull out the response payload and parse it
        let blob = response.payload
            .as_ref()
            .ok_or_else(|| DocGenError::ResponseParseError("empty payload".into()))?;
        let json: Value = serde_json::from_slice(blob.as_ref())
            .map_err(|e| DocGenError::ResponseParseError(e.to_string()))?;

        // 6) Extract the URL
        json.get("download_url")
            .and_then(Value::as_str)
            .map(str::to_owned)
            .ok_or_else(|| {
                DocGenError::ResponseParseError(format!(
                    "missing download_url in response: {:?}", json
                ))
            })
    }
}
