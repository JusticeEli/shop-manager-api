use std::{num::ParseIntError, rc::Rc};

use actix_web::{
    get, middleware::Logger, post, web::Data, App, HttpResponse, HttpServer, Responder,
};
use anchor_client::{
    solana_sdk::signature::read_keypair_file,
    solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey, signature::Keypair},
    Client, Cluster, Program,
};
use base58::FromBase58;
#[macro_use]
extern crate dotenv_codegen;

use dotenv::dotenv;
use env_logger::Env;
use lazy_static::lazy_static;
use log::{debug, info};
use modals::ShopState;
use tokio;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    lazy_static! {
        static ref shop_configurations: ShopConfigurations =
            match setup_environment_and_get_configurations() {
                Ok(configurations) => configurations,
                Err(e) => {
                    error!("{e:#}");
                    panic!("{e:#}");
                }
            };
    }

    request_airdrop_for_current_wallet(&shop_configurations);

    HttpServer::new(|| {
        let shop_state = match get_shop_state(&shop_configurations) {
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

fn request_airdrop_for_current_wallet(
    shop_configurations: &ShopConfigurations,
) -> ShopResult<String> {
    let program = try_get_program(&shop_configurations)
        .map_err(|e| errors::ShopCustomError::getCustomError(e))?;
    let payer = program.payer();
    let tx_id = routes::check_balance_of_fee_payer_and_airdrop(&program)?;
    Ok(tx_id)
}

fn setup_environment_and_get_configurations() -> ShopResult<ShopConfigurations> {
    let shop_configurations = get_environment_configurations()?;

    // setup our default logging format with levels according to .env
    env_logger::init();
    info!("environment setup complete");
    Ok(shop_configurations)
}

fn get_shop_state(shop_configurations: &ShopConfigurations) -> ShopResult<ShopState> {
    let program = try_get_program(shop_configurations)?;
    let shop_state = ShopState {
        shop_configurations: shop_configurations,
    };
    Ok(shop_state)
}
use std::env;

use crate::modals::ShopConfigurations;
/**
 * fetches host and port from .env file
 */
fn get_environment_configurations() -> ShopResult<ShopConfigurations> {
    dotenv().map_err(|e| errors::ShopCustomError::getCustomError(e))?;

    for (key, value) in env::vars() {
        info!("config:  Key:{key} Value:{value}");
    }

    let host = dotenv!("HOST");
    let port = dotenv!("PORT");
    let program_id = dotenv!("PROGRAM_ID");
    let cluster = dotenv!("CLUSTER");
    let cluster_url = dotenv!("CLUSTER_URL");
    let cluster_ws_url = dotenv!("CLUSTER_WS_URL");

    let optional_payer_key_pair = option_env!("PAYER_KEY_PAIR");
    let optional_account_key_pair = option_env!("ACCOUNT_PUBKEY");

    let payer_key_pair_bytes =
        get_key_pair_bytes_from_env_string_return_random_if_no_key_string_found(
            optional_payer_key_pair,
        );
    let account_key_pair_bytes =
        get_key_pair_bytes_from_env_string_return_random_if_no_key_string_found(
            optional_account_key_pair,
        );

    let configurations = ShopConfigurations {
        host: host.to_string(),
        port: port.to_string(),
        program_id: program_id.to_string(),
        cluster: cluster.to_string(),
        cluster_url: cluster_url.to_string(),
        cluster_ws_url: cluster_ws_url.to_string(),
        payer_key_pair_bytes,
        account_key_pair_bytes,
    };

    Ok(configurations)
}

lazy_static! {
    static ref goods_account_keypair1: Keypair = get_account_key_pair();
    static ref payer1: Keypair = Keypair::new();
}

fn get_account_key_pair() -> Keypair {
    return Keypair::new();
}
fn try_get_program(shop_configurations: &ShopConfigurations) -> ShopResult<Program> {
    let program_id = try_get_program_id(&shop_configurations.program_id)?;

    let cluster = get_cluster(shop_configurations);
    let payer_key_pair = keypair_from_bytes(&shop_configurations.payer_key_pair_bytes)?;
    let client = configure_and_get_client(cluster, payer_key_pair);

    let program = client.program(program_id);
    return Ok(program);
}

fn keypair_from_bytes(key_pair_bytes: &[u8]) -> ShopResult<Keypair> {
    let key_pair = Keypair::from_bytes(&key_pair_bytes)
        .map_err(|e| errors::ShopCustomError::getCustomError(e))?;
    Ok(key_pair)
}

fn configure_and_get_client(cluster: Cluster, payer_key_pair: Keypair) -> Client {
    let client = Client::new_with_options(
        cluster,
        Rc::new(payer_key_pair),
        CommitmentConfig::processed(),
    );
    client
}
use log::error;
fn get_key_pair_bytes_from_env_string_return_random_if_no_key_string_found(
    optional_string_key_pair: Option<&str>,
) -> [u8; 64] {
    info!("fetching keypair info...");
    match optional_string_key_pair {
        Some(key_pair_string) => {
            info!("success getting keypair from env");
            match try_parse_key_pair(key_pair_string) {
                Ok(key_pair) => key_pair.to_bytes(),
                Err(e) => {
                    error!("{e:#?}");
                    get_random_key_pair().to_bytes()
                }
            }
        }
        None => {
            info!(" key pair was not passed,falling back to a random keypair");

            get_random_key_pair().to_bytes()
        }
    }
}

fn get_random_key_pair() -> Keypair {
    return Keypair::new();
}
type ShopResult<T> = Result<T, Box<dyn Error>>;

fn try_parse_key_pair(key_pair_string: &str) -> ShopResult<Keypair> {
    //Vec<Result<u8,ParseIntError>>
    //Result<Vec<u8>,ParIntError>
    let key_pair_vec = key_pair_string
        .split(',')
        .map(|p| p.parse::<u8>())
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| errors::ShopCustomError::getCustomError(e))?;

    let key_pair = Keypair::from_bytes(&key_pair_vec)
        .map_err(|e| errors::ShopCustomError::getCustomError(e))?;

    Ok(key_pair)
}

fn get_cluster(shop_configurations: &ShopConfigurations) -> Cluster {
    let cluster: &str = &shop_configurations.cluster;

    match cluster {
        "localnet" => Cluster::Localnet,
        "devnet" => Cluster::Devnet,
        "custom" => {
            let url = Cluster::Custom(
                shop_configurations.cluster_url.to_string(),
                shop_configurations.cluster_ws_url.to_string(),
            );
            url
        }
        _ => Cluster::default(),
    }
}
use std::error::Error;
use std::fmt;
fn try_get_program_id(program_id_as_base_58: &str) -> ShopResult<Pubkey> {
    let program_id_vec = program_id_as_base_58
        .from_base58()
        .map_err(|e| errors::ShopCustomError(format!("{e:#?}")))?;
    let program_id = Pubkey::new(&program_id_vec);
    Ok(program_id)
}
extern crate derive_more;
use actix_web::error::ResponseError;
use derive_more::{Display, Error, From};
mod errors {
    use super::*;
    use std::fmt::Debug;
    #[derive(Debug)]
    pub struct ShopCustomError(pub String);

    impl ShopCustomError {
        pub fn getCustomError<E: Debug>(e: E) -> ShopCustomError {
            return Self(format!("{e:#?}"));
        }
    }
    impl fmt::Display for ShopCustomError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            writeln!(f, "{}", self.0)
        }
    }
    impl Error for ShopCustomError {}
    #[derive(Error, Display, Debug)]
    struct ShopResponseError(ShopCustomError);

    impl ResponseError for ShopCustomError {}
    impl ResponseError for ShopResponseError {}
}

mod routes {

    use super::*;
    use actix_web::web;
    use actix_web::web::Json;
    use anchor_client::anchor_lang::system_program;
    use anchor_client::anchor_lang::system_program::System;
    use anchor_client::solana_sdk::config::program;
    use anchor_client::solana_sdk::native_token::LAMPORTS_PER_SOL;
    use anchor_client::solana_sdk::signer::Signer;
    use log::debug;
    use log::info;
    use shop_manager::accounts;
    use shop_manager::instruction;
    use shop_manager::Good;
    use shop_manager::GoodsAccount;

    #[post("/initialize")]
    pub async fn initialize(shop_state: web::Data<ShopState<'static>>) -> Result<String> {
        // Build and send a transaction.

        // Process each socket concurrently.
        info!("transactions ongoing...");

        let goods_account_key_pair =
            keypair_from_bytes(&shop_state.shop_configurations.account_key_pair_bytes)?;

        let handle = std::thread::spawn(move || {
            let program = try_get_program(shop_state.shop_configurations)
                .map_err(|e| errors::ShopCustomError::getCustomError(e))?;
            let payer = program.payer();

            info!("payer:{payer}");

            let tx = program
                .request()
                .accounts(accounts::Initialize {
                    user: program.payer(),
                    system_program: system_program::ID,
                    goods_account: goods_account_key_pair.pubkey(),
                })
                .signer(&goods_account_key_pair)
                .args(instruction::Initialize)
                .send()
                .map(|r| return r.to_string())
                .map_err(|e| errors::ShopCustomError::getCustomError(e));

            return tx;
        });
        let tx_id = handle
            .join()
            .map_err(|e| errors::ShopCustomError::getCustomError(e))?
            .map_err(|e| errors::ShopCustomError::getCustomError(e))?;

        let result = format!("transaction signature:{tx_id}");
        info!("{}", result);
        Ok(tx_id)
    }

    #[post("/insert_goods")]
    pub async fn insert_goods(
        shop_state: web::Data<ShopState<'static>>,
        good: web::Json<Good>,
    ) -> Result<String> {
        let good = good.into_inner();
        info!("good:{good:?}");
        let goods_account_key_pair =
            keypair_from_bytes(&shop_state.shop_configurations.account_key_pair_bytes)?;

        info!("transactions ongoing...");

        let handle = std::thread::spawn(move || {
            let program = try_get_program(shop_state.shop_configurations)
                .map_err(|e| errors::ShopCustomError::getCustomError(e))?;
            let tx = program
                .request()
                .accounts(accounts::AddGoods {
                    goods_account: goods_account_key_pair.pubkey(),
                })
                .args(instruction::InsertGoods { good: good.clone() })
                .send()
                .map(|r| return r.to_string())
                .map_err(|e| errors::ShopCustomError::getCustomError(e));

            let goods_account: GoodsAccount = program
                .account(goods_account_key_pair.pubkey())
                .map_err(|e| errors::ShopCustomError::getCustomError(e))?;
            info!("goods_account: {goods_account:#?}");

            return tx;
        });
        let tx_id = handle
            .join()
            .map_err(|e| errors::ShopCustomError::getCustomError(e))?
            .map_err(|e| errors::ShopCustomError::getCustomError(e))?;

        //  debug!("tx_id:{tx_id}");

        Ok(tx_id.to_string())
    }
    use actix_web::Result;

    pub fn check_balance_of_fee_payer_and_airdrop(
        program: &Program,
    ) -> Result<String, errors::ShopCustomError> {
        let payer = program.payer();

        let tx_id = program
            .rpc()
            .request_airdrop(&payer, 50 * LAMPORTS_PER_SOL)
            .map_err(|e| errors::ShopCustomError::getCustomError(e))?;
        info!(
            "payer:{} has successfully received an airdrop of 3 SOL",
            payer
        );

        let confirm_transaction = program
            .rpc()
            .confirm_transaction(&tx_id)
            .map_err(|e| errors::ShopCustomError::getCustomError(e))?;
        info!("status of airdrop transaction:{}", confirm_transaction);

        let balance = program
            .rpc()
            .get_balance(&payer)
            .map_err(|e| errors::ShopCustomError::getCustomError(e))?;
        info!("payer id: {} current balance is :{}", payer, balance);

        Ok(tx_id.to_string())
    }
}

mod instructions {
    use super::*;
    pub struct Initialize {
        pub goods_account: Pubkey,
        pub user: Pubkey,
        pub system_program: Pubkey,
    }
}
mod modals {
    use anchor_client::anchor_lang::prelude::borsh::de;

    use super::*;

    pub struct ShopState<'a> {
        pub shop_configurations: &'a ShopConfigurations,
    }
    #[derive(Clone)]
    pub struct ShopConfigurations {
        pub host: String,
        pub port: String,
        pub program_id: String,
        pub cluster: String,
        pub cluster_url: String,
        pub cluster_ws_url: String,
        pub payer_key_pair_bytes: [u8; 64],
        pub account_key_pair_bytes: [u8; 64],
    }
}
#[cfg(test)]
mod tests {
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
        let shop_state = setup().await.unwrap();
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

    async fn setup() -> ShopResult<ShopState<'static>> {
        lazy_static! {
            static ref SHOP_CONFIGURATIONS: ShopConfigurations =
                setup_environment_and_get_configurations().unwrap();
        }
        let tx_id = tokio::task::spawn_blocking(|| -> Result<String, errors::ShopCustomError> {
            let tx_id = request_airdrop_for_current_wallet(&SHOP_CONFIGURATIONS)
                .map_err(|e| errors::ShopCustomError::getCustomError(e))?;
            std::thread::sleep(Duration::from_millis(500));
            Ok(tx_id)
        })
        .await??;
        info!("tx_id:{}",tx_id);
        let shop_state = get_shop_state(&SHOP_CONFIGURATIONS)?;

        Ok(shop_state)
    }
}
