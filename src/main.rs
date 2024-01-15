use std::net::TcpListener;
use the_news_letter::configuration::get_configuration;
use the_news_letter::startup::run;
use sqlx::{PgPool};
#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Cannot get configuration!");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("ERROR CONNECTING DATABSE USING PGPOOL");
    let address = format!("127.0.0.1:{}", configuration.app_port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}
