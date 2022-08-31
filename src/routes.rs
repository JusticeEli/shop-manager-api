use super::*;
use actix_web::delete;
use actix_web::put;
use actix_web::web;
use actix_web::web::Json;
use actix_web::Result;
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
pub async fn initialize(shop_state: web::Data<ShopState<'static>>) -> Result<&'static str> {
    // Build and send a transaction.

    // Process each socket concurrently.
    info!("transactions ongoing...");

    let tx = run_blocking(
        shop_state.shop_configurations,
        move |program, goods_account_key_pair| {
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
                .map_err(|e| errors::ShopCustomError::get_custom_error(e));

            return tx;
        },
    )?;

    Ok("success")
}

#[get("/goods")]
pub async fn get_all_goods(shop_state: web::Data<ShopState<'static>>) -> Result<Json<Vec<Good>>> {
    let goods = run_blocking(
        shop_state.shop_configurations,
        move |program, goods_account_key_pair| {
            let goods_account: GoodsAccount = program
                .account(goods_account_key_pair.pubkey())
                .map_err(|e| errors::ShopCustomError::get_custom_error(e))?;

            let goods = goods_account.goods;

            return Ok(goods);
        },
    )?;

    Ok(Json(goods))
}

#[post("/goods")]
pub async fn insert_goods(
    shop_state: web::Data<ShopState<'static>>,
    good: web::Json<Good>,
) -> Result<HttpResponse> {
    let original_good = good.into_inner();

    info!("good:{original_good:?}");
    let good = original_good.clone();
    let tx_id = run_blocking(
        shop_state.shop_configurations,
        move |program, goods_account_key_pair| {
            program
                .request()
                .accounts(accounts::AddGoods {
                    goods_account: goods_account_key_pair.pubkey(),
                })
                .args(instruction::InsertGoods { good: good.clone() })
                .send()
                .map(|r| return r.to_string())
                .map_err(|e| errors::ShopCustomError::get_custom_error(e))
        },
    )?;

    let location: &str = &format!("/goods/{}", original_good.id);

    let response = HttpResponse::Created()
        .insert_header(("Location", location))
        .json(original_good);
    Ok(response)
}

#[get("/goods/{id}")]
pub async fn get_specific_good(
    path: web::Path<u64>,
    shop_state: web::Data<ShopState<'static>>,
) -> Result<Json<Good>> {
    let goods = run_blocking(
        shop_state.shop_configurations,
        move |program, goods_account_key_pair| {
            let goods = program
                .account::<GoodsAccount>(goods_account_key_pair.pubkey())
                .map_err(|e| errors::ShopCustomError::get_custom_error(e))?
                .goods;
            return Ok(goods);
        },
    )?;

    let good_id = path.into_inner();
    let not_found = format!("Resource with id:{good_id} Not Found");
    let good = goods
        .iter()
        .find(|g| g.id == good_id)
        .ok_or(actix_web::error::ErrorNotFound(not_found))?;
    Ok(Json(good.clone()))
}

#[put("/goods/{id}")]
pub async fn update_goods(
    shop_state: web::Data<ShopState<'static>>,
    path: web::Path<u64>,
    good: web::Json<Good>,
) -> Result<Json<Good>> {
    let good = good.into_inner();
    info!("good:{good:?}");
    let good_to_update = good.clone();

    let tx_id = run_blocking(
        shop_state.shop_configurations,
        move |program, goods_account_key_pair| {
            let tx = program
                .request()
                .accounts(accounts::AddGoods {
                    goods_account: goods_account_key_pair.pubkey(),
                })
                .args(instruction::UpdateGoods {
                    good: good_to_update,
                })
                .send()
                .map(|r| return r.to_string())
                .map_err(|e| errors::ShopCustomError::get_custom_error(e))?;
            Ok(tx)
        },
    )?;

    Ok(Json(good))
}

#[delete("/goods/{id}")]
pub async fn delete_goods(
    shop_state: web::Data<ShopState<'static>>,
    path: web::Path<(u64)>,
) -> Result<HttpResponse, actix_web::error::Error> {
    info!("transactions ongoing...");

    let goods = run_blocking(
        shop_state.shop_configurations,
        |program, goods_account_key_pair| {
            let goods_account: GoodsAccount = program
                .account(goods_account_key_pair.pubkey())
                .map_err(|e| errors::ShopCustomError::get_custom_error(e))?;
            let goods = goods_account.goods;

            Ok(goods)
        },
    )?;

    let good_id = path.into_inner();
    let result = goods.iter().find(|g| g.id == good_id).ok_or({
        let not_found = format!("Resource with id:{good_id} Not Found");

        let e = actix_web::error::ErrorNotFound(not_found);
        e
    })?;

    let tx = run_blocking(
        shop_state.shop_configurations,
        move |program, goods_account_key_pair| {
            let tx = program
                .request()
                .accounts(accounts::AddGoods {
                    goods_account: goods_account_key_pair.pubkey(),
                })
                .args(instruction::DeleteGoods { good_id })
                .send()
                .map(|r| return r.to_string())
                .map_err(|e| errors::ShopCustomError::get_custom_error(e))?;

            return Ok(tx);
        },
    )?;

    Ok(HttpResponse::NoContent().finish())
}

#[delete("/goods")]
pub async fn delete_all_goods(
    shop_state: web::Data<ShopState<'static>>,
) -> Result<HttpResponse, actix_web::error::Error> {
    let tx = run_blocking(
        shop_state.shop_configurations,
        move |program, goods_account_key_pair| {
            let tx = program
                .request()
                .accounts(accounts::AddGoods {
                    goods_account: goods_account_key_pair.pubkey(),
                })
                .args(instruction::DeleteAllGoods)
                .send()
                .map(|r| return r.to_string())
                .map_err(|e| errors::ShopCustomError::get_custom_error(e))?;

            return Ok(tx);
        },
    )?;

    Ok(HttpResponse::NoContent().finish())
}
