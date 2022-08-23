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
        shop_solana_utils::keypair_from_bytes(&shop_state.shop_configurations.account_key_pair_bytes)?;

    let handle = std::thread::spawn(move || {
        let program = shop_anchor_utils::try_get_program(shop_state.shop_configurations)
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
        shop_solana_utils::keypair_from_bytes(&shop_state.shop_configurations.account_key_pair_bytes)?;

    info!("transactions ongoing...");

    let handle = std::thread::spawn(move || {
        let program = shop_anchor_utils::try_get_program(shop_state.shop_configurations)
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