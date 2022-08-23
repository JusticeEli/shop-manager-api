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
use log::error;
use log::{debug, info};
use modals::ShopState;
use tokio;

use std::error::Error;
use std::fmt;

extern crate derive_more;
use actix_web::error::ResponseError;
use derive_more::{Display, Error, From};

use std::env;



mod configure;
mod entrypoint;
mod errors;
mod modals;
mod routes;
mod shop_anchor_utils;
mod shop_solana_utils;
mod tests;

pub use configure::*;
pub use entrypoint::*;
pub use errors::*;
pub use modals::*;
pub use routes::*;
pub use shop_anchor_utils::*;
pub use shop_solana_utils::*;

type ShopResult<T> = Result<T, Box<dyn Error>>;



pub async fn configure_and_start_server()-> std::io::Result<()> {
    lazy_static! {
        static ref SHOP_CONFIGURATIONS: ShopConfigurations =
            match configure::setup_environment_and_get_configurations() {
                Ok(configurations) => configurations,
                Err(e) => {
                    error!("{e:#}");
                    panic!("{e:#}");
                }
            };
    }
    entrypoint::start_server(&SHOP_CONFIGURATIONS).await
}

