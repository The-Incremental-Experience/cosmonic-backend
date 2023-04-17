use chatlog::*;
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_httpclient::*;

#[allow(dead_code)]
mod chatlog;

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
        let client = HttpClientSender::new();
        let mut headers = HeaderMap::new();
        headers.insert("accept".to_string(), vec!["application/json".to_string()]);
        headers.insert("authorization".to_string(), vec!["Bearer wilJVepgbMNVHebtIy8hYVnAQhvoJu5Qkp9UQEW2".to_string()]);
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

        let response_body: &str = std::str::from_utf8(&cohere_response.body).unwrap();
        let mut processed_message = (&arg.body).to_owned();
        let match_result = response_body.match_indices("\"text\":\"").next();
        let match_end = response_body.match_indices("\"}],\"prompt\"").next();
        if match_result.is_some() && match_end.is_some() {
            processed_message = "{\"message\": \"".to_owned() + &response_body[(match_result.unwrap().0 + 8)..match_end.unwrap().0] + "\"}"
        }

        Ok(TransformMessageResponse {
            success: true,
            result: Some(processed_message),
        })
    }


    async fn get_messages(&self, _ctx: &Context) -> RpcResult<MessagesList> {
        Ok(vec![CanonicalChatMessage {
            body: "test message".to_string(),
            channel_name: "test channel".to_string(),
            id: "test id".to_string(),
            source_user: "test user".to_string(),
        }])
    }
}
