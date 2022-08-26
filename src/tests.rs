#![cfg(test)]

use std::{process::{Child, Stdio}, time::Duration, io::Read, str::from_utf8};

use self::test_utils::airdrop_to_current_wallet;

use super::*;
use actix_web::{
    body::MessageBody,
    dev::{AppConfig, Service, ServiceFactory, ServiceResponse},
    test, web, App,
};
use anchor_client::solana_client::client_error::reqwest::Request;
use base58::ToBase58;
use serde::__private::from_utf8_lossy;
use shop_manager::Good;
use std::process::Command;
use tokio::time::sleep;

#[actix_web::test]
async fn test_initialize_post() {
    let mut solana_test_validator = test_initialize_post_helper().await;
    tear_down(&mut solana_test_validator)
}
async fn test_initialize_post_helper() -> Child {
    let (mut solana_test_validator, shop_state) = init_test_service().await;

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

    return solana_test_validator;
}

fn tear_down(solana_test_validator: &mut Child) {
    solana_test_validator.kill();
}
pub async fn init_test_service() -> (Child, ShopState<'static>) {
    info!("init_test_service");
    let shop_state = test_utils::setup_configuration_and_return_state()
        .await
        .unwrap();

    let solana_test_validator = init_test_validator_and_deploy_program();
    airdrop_to_current_wallet(shop_state.shop_configurations).await;

    (solana_test_validator, shop_state)
}
fn init_test_validator_and_deploy_program() -> Child {
    info!("init_test_validator_and_deploy_program");
    Command::new("pkill")
        .arg("solana")
        .spawn()
        .expect("Failed to kill solana-test-validator");
    info!("pkill started...");
    std::thread::sleep(Duration::from_secs(1));

    let mut solana_test_validator = Command::new("solana-test-validator")
        .arg("--reset")
        .stdout(Stdio::piped())
        .spawn()
        .expect("solana-test-validator failed to execute");
    info!("solana_test_validator started...");




    std::thread::sleep(Duration::from_secs(7));

    Command::new("anchor")
        .arg("deploy")
        .current_dir("/home/justice/solana_projects/shop-manager")
        .spawn()
        .expect("Anchor deploy failed to deploy program to solana local validator");

    info!("anchor deploy started...");
    std::thread::sleep(Duration::from_secs(7));

    solana_test_validator
}
 #[actix_web::test]
async fn test_insert_goods_post() {
    let mut solana_test_validator = test_initialize_post_helper().await;
    let shop_state = test_utils::setup_configuration_and_return_state()
    .await
    .unwrap();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(shop_state))
            .service(routes::initialize)
            .service(routes::insert_goods),
    )
    .await;

    let good = Good {
        name: "unga".to_string(),
        image: "image1".to_string(),
        id: 1,
        price: 26,
    };
  
    let req = test::TestRequest::post()
        .uri("/insert_goods")
        .set_json(&good)
        .to_request();
    let goods_vec: Vec<Good> = test::call_and_read_body_json(&app, req).await;
    info!("goods_vec:{:?}", goods_vec);
    assert_eq!(goods_vec, vec![good]);
    
    tear_down(&mut solana_test_validator)

} 

mod test_utils {
    use super::*;
    use std::process::Command;
    pub async fn setup() -> ShopResult<ShopState<'static>> {
        // std::thread::sleep(Duration::from_secs(2));

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
        // info!("tx_id:{}", tx_id);
        let shop_state = configure::get_shop_state(&SHOP_CONFIGURATIONS)?;

        Ok(shop_state)
    }
    pub async fn setup_configuration_and_return_state() -> ShopResult<ShopState<'static>> {
        // std::thread::sleep(Duration::from_secs(2));

        lazy_static! {
            static ref SHOP_CONFIGURATIONS: ShopConfigurations =
                configure::setup_environment_and_get_configurations().unwrap();
        }

        let shop_state = configure::get_shop_state(&SHOP_CONFIGURATIONS)?;

        Ok(shop_state)
    }

    pub async fn airdrop_to_current_wallet(shop_configurations: &'static ShopConfigurations) {
        let tx_id = tokio::task::spawn_blocking(|| -> Result<String, errors::ShopCustomError> {
            let tx_id = shop_solana_utils::request_airdrop_for_current_wallet(shop_configurations)
                .map_err(|e| errors::ShopCustomError::getCustomError(e))?;
            info!("airdrop_to_current_wallet: tx_id:{}", tx_id);

            std::thread::sleep(Duration::from_millis(500));

            Ok(tx_id)
        })
        .await;
    }
}
