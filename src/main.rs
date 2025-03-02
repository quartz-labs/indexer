mod errors;
mod config;
use config::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::from_env()?;

    println!("RPC URL: {:?}", config.rpc_url);
    println!("Redis URL: {:?}", config.redis_url);
    
    Ok(())
}