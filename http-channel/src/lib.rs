use chatlog::*;
use outbound::*;
use serde::{Deserialize, Serialize};
use serde_json::json;
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
            ("OPTIONS", ["messages"]) => {
                let mut header = HeaderMap::new();
                header.insert("Access-Control-Allow-Origin".to_string(), vec!["".to_string()]);
                header.insert("Access-Control-Allow-Headers".to_string(), vec!["".to_string()]);
                header.insert("Access-Control-Allow-Methods".to_string(), vec!["GET, OPTIONS, PUT, POST, DELETE".to_string()]);

                return Ok(HttpResponse{
                    header,
                    status_code: 204,
                    ..Default::default()
                });
            },
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

    match logger
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
        {
            Ok(r) => {
                
                let mut res_headers = HeaderMap::new();

                res_headers.insert(
                    "Access-Control-Allow-Origin".to_string(),
                    vec!["*".to_string()],
                );
                res_headers.insert(
                    "Access-Control-Allow-Methods".to_string(),
                    vec!["GET".to_string(), "POST".to_string(), "PUT".to_string()],
                );
                res_headers.insert(
                    "Access-Control-Allow-Headers".to_string(),
                    vec!["*".to_string()],
                );
                //let mut headers: HeaderMap = HeaderMap::new();
                //headers.insert("ACCESS_CONTROL_ALLOW_ORIGIN".to_string(), vec!["*".to_string()]);
                let response: Result<HttpResponse, RpcError> =  HttpResponse::json_with_headers(r, 200,  res_headers);    
                response
            },
            Err(e) => Ok(HttpResponse::internal_server_error(format!("{}", e))),
        }
}

async fn get_messages(ctx: &Context) -> RpcResult<HttpResponse> {
    let logger = ChatlogSender::to_actor(API_ACTOR);
    match logger.get_messages(ctx).await {
        Ok(r) => {
            let mut response =  HttpResponse::json(r, 200);
            response
        }
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