use chatlog::*;
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_httpclient::*;

#[allow(dead_code)]
mod chatlog;

const TRANSLATION_URL: &str = "https://api.cognitive.microsofttranslator.com/translate?api-version=3.0&to=de";

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, Chatlog)]
struct ChatlogActor {}

/// Implementation of Chat Log actor trait methods
#[async_trait]
impl Chatlog for ChatlogActor {
    async fn transform_message(
        &self,
        ctx: &Context,
        arg: &CanonicalChatMessage,
    ) -> RpcResult<TransformMessageResponse> {

        let client = HttpClientSender::new();
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type".to_string(), vec!["application/json".to_string()]);
        headers.insert("Ocp-Apim-Subscription-Key".to_string(),  vec!["e27dc23e420944bbb0eb7cae2690a577".to_string()]);
       // headers.insert("X-ClientTraceId".to_string(),  vec!["1234".to_string()]); needs to be random, or maybe dont need at all
        headers.insert("Ocp-Apim-Subscription-Region".to_string(),  vec!["westeurope".to_string()]);


        let body = "[{'Text':'".to_owned() + &arg.body.to_owned() + "'}]";
        let request = HttpRequest{method:"POST".to_string(), url: TRANSLATION_URL.to_string(), headers: headers, body: body
        .as_bytes()
        .to_vec(),};
         
        // Translate the message
        let translation_response = client
            .request(
                ctx, &request).await?; 
         
       
        let mut processed_message = (&arg.body).to_owned(); 
        
        if translation_response.status_code == 200{
    
        let response_body = std::str::from_utf8(&translation_response.body).unwrap();
      
        let match_result = response_body.match_indices("\"text\":\"").next();
        let match_end = response_body.match_indices("\"}],\"to\"").next();
        if match_result.is_some() && match_end.is_some() {
            processed_message = "{\"message\": \"".to_owned() + &response_body[(match_result.unwrap().0 + 8)..match_end.unwrap().0] + "\"}"
        }
        
        }

        let response_body = std::str::from_utf8(&translation_response.body).unwrap();
      
        Ok(TransformMessageResponse {
            success: true,
            result: Some(response_body.to_string()),
        })
    }


    async fn get_messages(&self, _ctx: &Context) -> RpcResult<MessagesList> {
        Ok(vec![CanonicalChatMessage {
            body: "test message".to_string(),
            channel_name: "test channel".to_string(),
            id: "test id".to_string(),
            method: "smth".to_string(),
            source_user: "test user".to_string(),
        }])
    }
}
