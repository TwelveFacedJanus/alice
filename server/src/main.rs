use core::types::AliceResult;
use core::database_manager::DatabaseManager;
use core::connection::Connection;
use core::security::{AliceUser, AliceUserRole};

use futures::future;
use dotenv::dotenv;



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    env_logger::init();

    println!("Starting alice server.");
    let addr = std::env::var("GRPC_SERVER_ADDRESS")?.parse()?;

    Ok(())
}
