use chatlog::*;
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_httpclient::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use wasmcloud_interface_logging::{error, info};
use wasmcloud_interface_keyvalue::*;
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

        let kv = KeyValueSender::new();
        let key = kv.get(ctx, "CHATGPT_KEY").await?;

        let client = HttpClientSender::new();
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type".to_string(), vec!["application/json".to_string()]);
        headers.insert("Authorization".to_string(),  vec![key.value]);
       // headers.insert("X-ClientTraceId".to_string(),  vec!["1234".to_string()]); needs to be random, or maybe dont need at all
       

       let body = "{
        \"model\": \"gpt-3.5-turbo\",
        \"messages\": [{\"role\": \"user\", \"content\": \"".to_owned() + &arg.body.to_owned() + "\"}, {\"role\": \"assistant\",  \"content\": \"Correct grammatical mistakes and reformulate better the text I just gave you \"}],
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
            //info!("{:?}", response_body.clone());
            let chat_completion: ChatCompletion = serde_json::from_str(response_body).unwrap();
            //info!("{:?}", chat_completion.choices[0].message.content);
            processed_message = chat_completion.choices[0].message.content.clone();
            info!("here, {:?}", processed_message);
            
        }
        
        info!("here, {:?}", processed_message);
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
    choices: Vec<Choice>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Choice {
    message: Message,
}

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    content: String,
}