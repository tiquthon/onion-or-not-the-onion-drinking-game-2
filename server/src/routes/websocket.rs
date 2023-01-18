use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_ws::Message;
use std::sync::Arc;

use futures_util::StreamExt;

#[tracing::instrument(name = "Websocket", skip(req, body))]
pub async fn ws(
    req: HttpRequest,
    body: web::Payload,
    server: web::Data<Arc<tokio::sync::Mutex<crate::model::Server>>>,
) -> Result<HttpResponse, Error> {
    let (response, mut session, mut msg_stream) = actix_ws::handle(&req, body)?;

    actix_web::rt::spawn(async move {
        while let Some(Ok(msg)) = msg_stream.next().await {
            match msg {
                Message::Binary(bytes) => {
                    println!("Got binary, {bytes:?} ({})", bytes.len());
                }
                Message::Close(optional_close_reason) => {
                    tracing::info!(
                        "WebSocket connection closed by client ({optional_close_reason:?})"
                    );
                    return;
                }
                Message::Ping(bytes) => {
                    if let Err(error) = session.pong(&bytes).await {
                        tracing::info!("Connection closed while sending pong ({error})");
                        return;
                    }
                }
                Message::Continuation(item) => {
                    // TODO: Maybe don't ignore
                    // "Websocket protocol continuation frame" https://stackoverflow.com/a/25409934
                    tracing::warn!("Got Continuation Frame with data ({item:?})");
                }
                Message::Text(_) | Message::Pong(_) | Message::Nop => {
                    // IGNORE
                }
            }
        }

        tracing::info!("Closing WebSocket connection, because received None message");
        let _ = session.close(None).await;
    });

    Ok(response)
}
