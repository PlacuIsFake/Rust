mod network_manager;
use std::error::Error;

use network_manager::server::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let server = Server::new();
    server.start().await?;
    Ok(())
}
