use chatlog::*;
use outbound::{Outbound, OutboundMessage, OutboundSender};
use serde::Deserialize;
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_logging::{error, info};
use serde_json;

#[allow(dead_code)]
mod chatlog;

#[allow(dead_code)]
mod outbound;

const CHATGPT_ACTOR: &str = "mcchat/chatgpt";
const COHERE_ACTOR: &str = "mcchat/cohere";
const CHATLOG_ACTOR: &str = "mcchat/chatlog";

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, Chatlog)]
struct ApiGatewayActor {}

const KNOWN_CHANNEL_NAMES: &[&str] = &["http", "nats"];


/// Implementation of Chat Log actor trait methods
#[async_trait]
impl Chatlog for ApiGatewayActor {
    async fn transform_message(
        &self,
        ctx: &Context,
        arg: &CanonicalChatMessage,
    ) -> RpcResult<TransformMessageResponse> {
        
        info!("{:?}", arg.clone());
        //let body: serde_json::Value = serde_json::from_str(&arg.body).unwrap();
        // todo: use body["method"].as_str().unwrap() instead of "prettify"

        let actor_id: &str = self.get_routing(&arg.method).await;
        //let mut arg2 = arg.clone();
        // todo: use body["body"] instead of "sheep"
        //arg2.body = "sheep".to_owned();

        let service_actor = ChatlogSender::to_actor(actor_id);
        service_actor.transform_message(ctx, arg).await
    }

    // TODO: delete this in the end, it is only for debugging
    async fn get_messages(&self, ctx: &Context) -> RpcResult<MessagesList> {
        let chatlog = ChatlogSender::to_actor(CHATLOG_ACTOR);

        chatlog.get_messages(ctx).await
    }
}

impl ApiGatewayActor {
    async fn get_routing(&self, mut method: &str) -> &str {
        if method.is_empty() {
            // default
            method = "translate";
        }
        match method {
            "translate" => CHATLOG_ACTOR,
            "prettify" => COHERE_ACTOR,
            _ => COHERE_ACTOR
        }
    }
}