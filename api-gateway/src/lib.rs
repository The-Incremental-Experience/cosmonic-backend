use chatlog::*;
use outbound::{Outbound, OutboundMessage, OutboundSender};
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_logging::error;

#[allow(dead_code)]
mod chatlog;

#[allow(dead_code)]
mod outbound;

const CHATLOG_ACTOR:&str = "mcchat/chatlog";
const COHERE_ACTOR:&str = "mcchat/cohere";

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

        // TODO: add here to logic to decide which actor to call ( currently it always calls chatlog actor - > a misleading title for translator)
        // let chatlog = ChatlogSender::to_actor(CHATLOG_ACTOR);
        // let res = chatlog.transform_message(ctx, arg).await;

        let cohere = ChatlogSender::to_actor(COHERE_ACTOR);
        let res = cohere.transform_message(ctx, arg).await;

        res
    }

    // TODO: delete this in the end, it is only for debugging
    async fn get_messages(&self, ctx: &Context) -> RpcResult<MessagesList> {
        // let chatlog = ChatlogSender::to_actor(CHATLOG_ACTOR);

        let cohere = ChatlogSender::to_actor(COHERE_ACTOR);

        cohere.get_messages(ctx).await
    }
}
