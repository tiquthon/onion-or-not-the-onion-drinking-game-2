use std::net::TcpListener;
use std::sync::Arc;

use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};

use tracing_actix_web::TracingLogger;

use crate::configuration::Configuration;
use crate::model;
use crate::routes::websocket::ws;

pub struct Application {
    #[allow(dead_code)]
    port: u16,
    server: Server,
}

impl Application {
    pub async fn build(configuration: Configuration) -> anyhow::Result<Self> {
        let application_address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );
        let tcp_listener = TcpListener::bind(&application_address)?;
        let port = tcp_listener.local_addr().unwrap().port();
        let server = run(tcp_listener).await?;
        Ok(Self { port, server })
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

async fn run(tcp_listener: TcpListener) -> anyhow::Result<Server> {
    let server_data = Arc::new(tokio::sync::Mutex::new(model::Server::new()));
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/ws", web::get().to(ws))
            .app_data(web::Data::new(Arc::clone(&server_data)))
    })
    .listen(tcp_listener)?
    .run();
    Ok(server)
}
