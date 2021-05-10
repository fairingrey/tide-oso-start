mod endpoints;
mod expenses;
mod middleware;
mod models;
mod server;

#[async_std::main]
async fn main() -> tide::Result<()> {
    dotenv::dotenv().ok();
    if let Err(err) = server::run().await {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
    Ok(())
}
