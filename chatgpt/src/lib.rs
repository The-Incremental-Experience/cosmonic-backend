use chatlog::*;
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_httpclient::*;

#[allow(dead_code)]
mod chatlog;


const TRANSLATION_URL: &str = "https://api.openai.com/v1/chat/completions";

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, Chatlog)]
struct ChatgptActor {}

/// Implementation of Chat Log actor trait methods
#[async_trait]
impl Chatlog for ChatgptActor {
    async fn transform_message(
        &self,
        ctx: &Context,
        arg: &CanonicalChatMessage,
    ) -> RpcResult<TransformMessageResponse> {

        let client = HttpClientSender::new();
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type".to_string(), vec!["application/json".to_string()]);
        headers.insert("Authorization".to_string(),  vec!["Bearer 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".to_string()]);
       // headers.insert("X-ClientTraceId".to_string(),  vec!["1234".to_string()]); needs to be random, or maybe dont need at all
       

       let body = "{
        \"model\": \"gpt-3.5-turbo\",
        \"messages\": [{\"role\": \"user\", \"content\": \"".to_owned() + &arg.body.to_owned() + "\"}, {\"role\": \"assistant\",  \"content\": \"Correct grammatical mistakes and reformulate better the text I give you \"}],
        \"temperature\": 0.7
      }";  

    

        let request = HttpRequest{method:"POST".to_string(), url: TRANSLATION_URL.to_string(), headers: headers, body: body
        .as_bytes()
        .to_vec(),};
         
        // Translate the message
        let translation_response = client
            .request(
                ctx, &request).await?; 
         
       
        let mut processed_message = (&arg.body).to_owned(); 
        
        if (translation_response.status_code == 200){
            let response_body = std::str::from_utf8(&translation_response.body).unwrap();

            let match_result = response_body.match_indices("\"content\":\"").next();
            let match_end = response_body.match_indices("\"}],\"finish_reason\"").next();
            if match_result.is_some() && match_end.is_some() {
            processed_message = "{\"message\": \"".to_owned() + &response_body[(match_result.unwrap().0 + 8)..match_end.unwrap().0] + "\"}"
        }
        
        }
        
        Ok(TransformMessageResponse {
            success: true,
            result: Some(processed_message.to_string()),
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
