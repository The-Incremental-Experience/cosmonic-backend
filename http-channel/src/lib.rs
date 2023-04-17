use chatlog::*;
use outbound::*;
use serde::{Deserialize, Serialize};
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_httpserver::*;
use wasmcloud_interface_logging::debug;
use wasmcloud_interface_numbergen::*;

#[allow(dead_code)]
mod chatlog;

#[allow(dead_code)]
mod outbound;

const CHANNEL_NAME: &str = "http";
const API_ACTOR: &str = "mcchat/api";

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, HttpServer, Outbound)]
struct HttpChannelActor {}

#[async_trait]
impl Outbound for HttpChannelActor {
    async fn publish_message(&self, ctx: &Context, arg: &OutboundMessage) -> RpcResult<bool> {
        // This is absorbed silently because the HTTP channel does not currently expose
        // any kind of realtime subscription. Perhaps in the future a websocket subscription
        // could be used?
        Ok(true)
    }
}
#[async_trait]
impl HttpServer for HttpChannelActor {
    async fn handle_request(&self, ctx: &Context, req: &HttpRequest) -> RpcResult<HttpResponse> {
        let path = &req.path[1..req.path.len()];
        let segments: Vec<&str> = path.trim_end_matches('/').split('/').collect();
        match (req.method.as_ref(), segments.as_slice()) {
            ("POST", ["messages"]) => transform_message(ctx, deser(&req.body)?).await,
            ("GET", ["messages"]) => get_messages(ctx).await,
            (m, p) => {
                debug!("unexpected method and path: {} - {:?}", m, p);
                Ok(HttpResponse::not_found())
            }
        }
    }
}

async fn transform_message(ctx: &Context, im: IncomingMessage) -> RpcResult<HttpResponse> {
    let logger = ChatlogSender::to_actor(API_ACTOR);
    let numgen = NumberGenSender::new();
    let guid = numgen.generate_guid(ctx).await.unwrap_or("n/a".to_string());

    logger
        .transform_message(
            ctx,
            &CanonicalChatMessage {
                body: im.body.body,
                channel_name: CHANNEL_NAME.to_string(),
                id: guid,
                method: im.method,
                source_user: im.body.user_name,
            },
        )
        .await
        .map(|r| r.into())
}

async fn get_messages(ctx: &Context) -> RpcResult<HttpResponse> {
    let logger = ChatlogSender::to_actor(API_ACTOR);
    match logger.get_messages(ctx).await {
        Ok(r) => HttpResponse::json(r, 200),
        Err(e) => Ok(HttpResponse::internal_server_error(format!("{}", e))),
    }
}

fn deser<'de, T: Deserialize<'de>>(raw: &'de [u8]) -> RpcResult<T> {
    serde_json::from_slice(raw).map_err(|e| RpcError::Deser(format!("{}", e)))
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct IncomingMessage {
    method: String,
    body: IncomingMessageinner,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
struct IncomingMessageinner {
    user_name: String,
    body: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Ack {
    accepted: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    reason: Option<String>,
}

impl From<chatlog::TransformMessageResponse> for HttpResponse {
    fn from(source: chatlog::TransformMessageResponse) -> Self {
        if source.success {
            let mut response = HttpResponse::default();
            response.body = source.result.unwrap_or_else(|| "".to_string()).as_bytes()
            .to_vec();
            response
        } else {
            HttpResponse::internal_server_error(source.result.unwrap_or_else(|| "".to_string()))
        }
    }
}