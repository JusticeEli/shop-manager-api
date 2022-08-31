use anchor_client::solana_sdk::signer::Signer;
use shop_manager::{accounts, instruction, Good, GoodsAccount};

use super::*;

pub fn run_blocking<T, F>(
    shop_configurations: &'static ShopConfigurations,
   // goods_account_key_pair: Keypair,
   // good: Option<&Good>,
    f: F,
) -> Result<T, ShopCustomError>
where
    F: FnOnce(Program, Keypair) -> Result<T, ShopCustomError>,
    F: Send + 'static,
    T: Send + 'static,
{
    let handle = std::thread::spawn(move || -> Result<T, ShopCustomError> {
        let program = shop_anchor_utils::try_get_program(shop_configurations)
            .map_err(|e| errors::ShopCustomError::get_custom_error(e))?;

            let goods_account_key_pair = shop_solana_utils::keypair_from_bytes(
                &shop_configurations.account_key_pair_bytes,
            )?;
        

        return f(program, goods_account_key_pair); //returns a result
    });

    let result = handle
        .join()
        .map_err(|e| errors::ShopCustomError::get_custom_error(e))?;
    return result;
}
