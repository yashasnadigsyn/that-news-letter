use std::net::TcpListener;
use the_news_letter::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("ERROR WHILE TCP LISTENER (health_check.rs 20)");
    run(listener)?.await
}
