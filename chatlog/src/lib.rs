use chatlog::*;
use serde::{Serialize, Deserialize};
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_httpclient::*;
use wasmcloud_interface_logging::info;

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
            info!(" HEREEE {:?}", response_body.clone());
            let chat_completion:  Vec<ChatCompletion> = serde_json::from_str(response_body).unwrap();
            info!("HEREEEE {:?}", chat_completion);
            processed_message = chat_completion[0].translations[0].text.clone();
        
        }

        //let response_body = std::str::from_utf8(&translation_response.body).unwrap();
      
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
            method: "smth".to_string(),
            source_user: "test user".to_string(),
        }])
    }
}


// Define a struct to represent the JSON response
#[derive(Debug, Serialize, Deserialize)]
struct ChatCompletion {
    translations: Vec<Choice>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Choice {
    text: String,
}
