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
