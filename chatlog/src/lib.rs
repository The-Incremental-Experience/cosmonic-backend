use chatlog::*;
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_httpclient::*;

#[allow(dead_code)]
mod chatlog;

mod store;
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
         
       
        let mut translation_body = "sheep";
        
        if (translation_response.status_code == 200){
            translation_body = std::str::from_utf8(&translation_response.body).unwrap();
        
        }

  
        let mut arg2 = arg.clone();
        arg2.body = translation_body.to_string();
    

        Ok(match store::write_message(ctx, &arg2).await {
            Ok(_) => TransformMessageResponse {
                success: true,
                result: Some(translation_body.to_string()),
            },
            Err(e) => TransformMessageResponse {
                success: false,
                result: None,
            },
        })
    }


    async fn get_messages(&self, ctx: &Context) -> RpcResult<MessagesList> {
        Ok(match store::get_messages(ctx).await {
            Ok(v) => v,
            Err(_) => vec![],
        })
    }
}
