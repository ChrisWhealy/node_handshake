use node_handshake::error::Result;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    node_handshake::start_handshakes().await
}
