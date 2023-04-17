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

        let body = "{\"max_tokens\": 20, \"return_likelihoods\": \"NONE\", \"truncate\": \"END\", \"prompt\": \"".to_owned() + &arg.body.to_owned() + "\"}";
        let request = HttpRequest {
            method: "POST".to_string(),
            url: COHERE_URL.to_string(),
            headers,
            body: Vec::from(body),
        };

        // Translate the message
        let cohere_response = client
            .request(
                ctx, &request).await?;


        let mut translation_body: String = cohere_response.status_code.to_string();
        translation_body.push_str(std::str::from_utf8(&cohere_response.body).unwrap());


        let mut arg2 = arg.clone();
        arg2.body = translation_body.to_string();


        Ok(TransformMessageResponse {
            success: true,
            result: Some(translation_body.to_string()),
        })
    }


    async fn get_messages(&self, ctx: &Context) -> RpcResult<MessagesList> {
        Ok(vec![])
    }
}
