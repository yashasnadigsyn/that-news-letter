use std::net::TcpListener;
use the_news_letter::run;
use the_news_letter::configuration::get_configuration;
#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Cannot get configuration!");
    let address = format!("127.0.0.1:{}", configuration.app_port);
    let listener = TcpListener::bind(address)?;
    run(listener)?.await
}
