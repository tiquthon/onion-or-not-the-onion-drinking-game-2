use std::net::TcpListener;

use actix_web::dev::Server;
use actix_web::{App, HttpServer};

use tracing_actix_web::TracingLogger;

use crate::configuration::Configuration;

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
    let server = HttpServer::new(move || App::new().wrap(TracingLogger::default()))
        .listen(tcp_listener)?
        .run();
    Ok(server)
}
