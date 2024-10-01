use rust_chat::server::Server

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    env_logger::init()
    Server::new(8080).run().await?;
Ok(())
}
