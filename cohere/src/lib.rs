use chatlog::*;
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_httpclient::*;
use serde::{Deserialize, Serialize};
use wasmcloud_interface_keyvalue::*;
#[allow(dead_code)]
mod chatlog;
mod store;

const COHERE_URL: &str = "https://api.cohere.ai/v1/generate";

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, Chatlog)]
struct CohereActor {}

/// Implementation of Chat Log actor trait methods
#[async_trait]
impl Chatlog for CohereActor {
    async fn transform_message(
        &self,
        ctx: &Context,
        arg: &CanonicalChatMessage,
    ) -> RpcResult<TransformMessageResponse> {

        let kv = KeyValueSender::new();
        let key = kv.get(ctx, "COHERE_KEY").await?;

        let client = HttpClientSender::new();
        let mut headers = HeaderMap::new();
        headers.insert("accept".to_string(), vec!["application/json".to_string()]);
        headers.insert("authorization".to_string(), vec![key.value]);
        headers.insert("content-type".to_string(), vec!["application/json".to_string()]);

        let body = "{\"max_tokens\": 20, \"return_likelihoods\": \"NONE\", \"truncate\": \"END\", \"prompt\": \"".to_owned()
            + &arg.body.to_owned()
            + "\"}";
        let request = HttpRequest {
            method: "POST".to_string(),
            url: COHERE_URL.to_string(),
            headers,
            body: Vec::from(body),
        };

        // Process the message
        let cohere_response = client
            .request(
                ctx, &request).await?;

        let mut processed_message = (&arg.body).to_owned(); 
        
        if (cohere_response.status_code == 200){
                    
        let response_body = std::str::from_utf8(&cohere_response.body).unwrap();
                    //info!("{:?}", response_body.clone());
        let chat_completion: ChatCompletion = serde_json::from_str(response_body).unwrap();
                    //info!("{:?}", chat_completion.choices[0].message.content);
        processed_message = chat_completion.generations[0].text.clone();
}

        Ok(TransformMessageResponse {
            success: true,
            result: Some(processed_message.to_string()),
        })
    }


    async fn get_messages(&self, ctx: &Context) -> RpcResult<MessagesList> {
        Ok(match store::get_messages(ctx).await {
            Ok(v) => v,
            Err(_) => vec![],
        })
    }
}

// Define a struct to represent the JSON response
#[derive(Debug, Serialize, Deserialize)]
struct ChatCompletion {
    generations: Vec<Choice>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Choice {
    text: String,
}

