use super::*;
pub fn keypair_from_bytes(key_pair_bytes: &[u8]) -> ShopResult<Keypair> {
    let key_pair = Keypair::from_bytes(&key_pair_bytes)
        .map_err(|e| errors::ShopCustomError::getCustomError(e))?;
    Ok(key_pair)
}


use anchor_client::solana_sdk::native_token::LAMPORTS_PER_SOL;
use log::error;
pub fn get_key_pair_bytes_from_env_string_return_random_if_no_key_string_found(
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

pub fn get_random_key_pair() -> Keypair {
    return Keypair::new();
}

pub fn try_get_program_id(program_id_as_base_58: &str) -> ShopResult<Pubkey> {
    let program_id_vec = program_id_as_base_58
        .from_base58()
        .map_err(|e| errors::ShopCustomError(format!("{e:#?}")))?;
    let program_id = Pubkey::new(&program_id_vec);
    Ok(program_id)
}
pub fn request_airdrop_for_current_wallet(
    shop_configurations: &ShopConfigurations,
) -> ShopResult<String> {
    info!("request_airdrop_for_current_wallet");
    let program = try_get_program(&shop_configurations)
        .map_err(|e| errors::ShopCustomError::getCustomError(e))?;
    let payer = program.payer();
    let tx_id = check_balance_of_fee_payer_and_airdrop(&program)?;
    Ok(tx_id)
}


pub fn try_parse_key_pair(key_pair_string: &str) -> ShopResult<Keypair> {
    
    let key_pair_vec = key_pair_string
        .split(',')
        .map(|p| p.parse::<u8>())
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| errors::ShopCustomError::getCustomError(e))?;

    let key_pair = Keypair::from_bytes(&key_pair_vec)
        .map_err(|e| errors::ShopCustomError::getCustomError(e))?;

    Ok(key_pair)
}

pub fn check_balance_of_fee_payer_and_airdrop(
    program: &Program,
) -> Result<String, errors::ShopCustomError> {

info!("check_balance_of_fee_payer_and_airdrop");
    const SOLS:u64=50;
    let payer = program.payer();
    let tx_id = program
        .rpc()
        .request_airdrop(&payer, SOLS * LAMPORTS_PER_SOL)
        .map_err(|e| errors::ShopCustomError::getCustomError(e))?;
    info!(
        "payer:{} has successfully received an airdrop of {} SOLS",
        payer,SOLS
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