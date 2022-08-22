use std::rc::Rc;

use actix_web::{
    get, middleware::Logger, post, web::Data, App, HttpResponse, HttpServer, Responder,
};
use anchor_client::{
    solana_sdk::signature::read_keypair_file,
    solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey, signature::Keypair},
    Client, Cluster, Program,
};
use base58::FromBase58;
use dotenv::dotenv;
use env_logger::Env;
use lazy_static::lazy_static;
use log::debug;
use tokio;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info,actix_server=info,actix_web=info");
    dotenv().ok();
    env_logger::init();

    
    debug!("program is starting");
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .app_data(Data::new(get_program()))
            .app_data(get_account_key_pair())
            .service(routes::insert_goods)
            .service(routes::initialize)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

lazy_static! {
    static ref goods_account_keypair: Keypair = get_account_key_pair();
    static ref payer: Keypair = Keypair::new();
}

fn get_account_key_pair() -> Keypair {
    return Keypair::new();
}
fn get_program() -> Program {
    let program_id = Pubkey::new(
        &"8agPo1zq2ZvXLqsgH5RuhxFJGJrsPTSSopZiYixYJXZy"
            .from_base58()
            .unwrap(),
    );
    let url = Cluster::Custom(
        "http://localhost:8899".to_string(),
        "ws://127.0.0.1:8900".to_string(),
    );
    let key_pair = Keypair::from_bytes(&payer.to_bytes()).unwrap();
    // Client.
    let client = Client::new_with_options(url, Rc::new(key_pair), CommitmentConfig::processed());

    let program1 = client.program(program_id);
    return program1;
}

mod routes {
    use super::*;
    use actix_web::web;
    use actix_web::web::Json;
    use anchor_client::anchor_lang::system_program;
    use anchor_client::anchor_lang::system_program::System;
    use anchor_client::solana_sdk::config::program;
    use anchor_client::solana_sdk::signer::Signer;
    use log::debug;
    use log::info;
    use shop_manager::accounts;
    use shop_manager::instruction;
    use shop_manager::Good;

    #[post("/insert_goods")]
    pub async fn insert_goods(
        program: web::Data<Program>,
        good: web::Json<Good>,
    ) -> impl Responder {
        let good = good.into_inner();
        debug!("good:{good:?}");
        // Build and send a transaction.

        // Process each socket concurrently.
        info!("transactions ongoing...");

        let handle = std::thread::spawn(move || {
            let tx_id = run_tx(good.clone());
        });
        handle.join();

        //  debug!("tx_id:{tx_id}");

        HttpResponse::Ok().body("success")
    }
    #[post("/initialize")]
    pub async fn initialize(program: web::Data<Program>) -> impl Responder {
        // Build and send a transaction.

        // Process each socket concurrently.
        info!("transactions ongoing...");

        info!("payer:{}", program.payer().to_string());

        let handle = std::thread::spawn(move || {
            let program = get_program();
            let tx_id = program
                .request()
                .accounts(accounts::Initialize {
                    user: program.payer(),
                    system_program: system_program::ID,
                    goods_account: goods_account_keypair.pubkey(),
                })
                .signer(&Keypair::from_bytes(&goods_account_keypair.to_bytes()).unwrap())
                .args(instruction::Initialize)
                .send()
                .unwrap_err();

            return tx_id;
        });
        let tx_id = handle.join().unwrap();

        let result = format!("transaction signature:{tx_id}");
        info!("{}", result);
        HttpResponse::Ok().body(result)
    }

    fn run_tx(good: Good) -> String {
        let program = get_program();
        let tx_id = program
            .request()
            .accounts(accounts::AddGoods {
                goods_account: goods_account_keypair.pubkey(),
            })
            .args(instruction::InsertGoods { good: good.clone() })
            .send()
            .unwrap();

        return "".to_owned();
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
    use super::*;

    pub struct ShopState {
        pub program: Program,
    }
}
