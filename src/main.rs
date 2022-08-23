use shop_manager_api;


#[tokio::main]
async fn main() -> std::io::Result<()> {
  
    shop_manager_api::configure_and_start_server().await
}
