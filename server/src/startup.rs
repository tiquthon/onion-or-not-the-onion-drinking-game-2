use std::net::TcpListener;

use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};

use tracing_actix_web::TracingLogger;

use crate::configuration::Configuration;
use crate::routes::game::lobbies_storage::LobbiesStorage;
use crate::routes::game::{create_lobby, join_lobby};

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
    let lobbies_storage = LobbiesStorage::default();
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/create", web::get().to(create_lobby))
            .route("/join/{invite_code}", web::get().to(join_lobby))
            .app_data(web::Data::new(lobbies_storage.clone()))
    })
    .listen(tcp_listener)?
    .run();
    Ok(server)
}
