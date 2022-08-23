
use super::*;

pub async fn start_server(shop_configurations:&'static ShopConfigurations)->std::io::Result<()>{
    shop_solana_utils::request_airdrop_for_current_wallet(&shop_configurations);
    HttpServer::new( move || {
        let shop_state = match configure::get_shop_state(&shop_configurations) {
            Ok(shop_state) => shop_state,
            Err(e) => {
                error!("{e:#}");
                panic!("{e:#}");
            }
        };

        App::new()
            .wrap(Logger::default())
            .app_data(Data::new(shop_state))
            .service(routes::initialize)
            .service(routes::insert_goods)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}




