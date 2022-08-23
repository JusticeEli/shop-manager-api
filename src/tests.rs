
#![cfg(test)]
 
    use std::time::Duration;

    use super::*;
    use actix_web::{
        body::MessageBody,
        dev::{Service, ServiceResponse},
        test, web, App,
    };
    use anchor_client::solana_client::client_error::reqwest::Request;
    use base58::ToBase58;
    use tokio::time::sleep;

    #[actix_web::test]
    async fn test_initialize_post() {
        let shop_state = stest_utils::etup().await.unwrap();
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(shop_state))
                .service(routes::initialize)
                .service(routes::insert_goods),
        )
        .await;

        let req = test::TestRequest::post().uri("/initialize").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
        let bytes = resp.into_body().try_into_bytes().unwrap();
        let tx_id = bytes.to_base58();
        info!("signature:{}", tx_id);
    }

    mod test_utils{
       pub async fn setup() -> ShopResult<ShopState<'static>> {
            lazy_static! {
                static ref SHOP_CONFIGURATIONS: ShopConfigurations =
                    configure::setup_environment_and_get_configurations().unwrap();
            }
            let tx_id = tokio::task::spawn_blocking(|| -> Result<String, errors::ShopCustomError> {
                let tx_id = shop_solana_utils::request_airdrop_for_current_wallet(&SHOP_CONFIGURATIONS)
                    .map_err(|e| errors::ShopCustomError::getCustomError(e))?;
                std::thread::sleep(Duration::from_millis(500));
                Ok(tx_id)
            })
            .await??;
            info!("tx_id:{}", tx_id);
            let shop_state = configure::get_shop_state(&SHOP_CONFIGURATIONS)?;
    
            Ok(shop_state)
        }
    }

  

