use anyhow::Context;

use onion_or_not_the_onion_drinking_game_2_server::configuration::{
    get_configuration, Environment,
};
use onion_or_not_the_onion_drinking_game_2_server::startup::Application;
use onion_or_not_the_onion_drinking_game_2_server::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing aka logging
    let subscriber = get_subscriber("debug".into(), std::io::stdout);
    init_subscriber(subscriber);

    // Build and run application
    let configuration =
        get_configuration::<Environment, _>().context("Failed to read configuration")?;
    Application::build(configuration)
        .await?
        .run_until_stopped()
        .await?;

    Ok(())
}

/* TODO: use actix_web::{middleware::Logger, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_ws::Message;

use futures_util::StreamExt;

async fn ws(req: HttpRequest, body: web::Payload) -> Result<HttpResponse, Error> {
    let (response, mut session, mut msg_stream) = actix_ws::handle(&req, body)?;

    actix_web::rt::spawn(async move {
        while let Some(Ok(msg)) = msg_stream.next().await {
            match msg {
                Message::Ping(bytes) => {
                    if session.pong(&bytes).await.is_err() {
                        return;
                    }
                }
                Message::Text(s) => println!("Got text, {s}"),
                _ => break,
            }
        }

        let _ = session.close(None).await;
    });

    Ok(response)
}

async fn hello() -> String {
    "Hello world!".to_owned()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .route("/", web::get().to(hello))
            .route("/ws", web::get().to(ws))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
*/
