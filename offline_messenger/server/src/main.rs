mod network_manager;

use network_manager::server::Server;

#[tokio::main]
async fn main() {
    let server = Server::new();
    match server.start().await {
        Ok(_) => {},
        Err(err) => {
            println!("Error while starting the server: {err}");
        },
    }
    
}
