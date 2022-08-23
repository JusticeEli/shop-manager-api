
use super::*;
pub fn get_environment_configurations() -> ShopResult<ShopConfigurations> {
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
        shop_solana_utils::get_key_pair_bytes_from_env_string_return_random_if_no_key_string_found(
            optional_payer_key_pair,
        );
    let account_key_pair_bytes =
        shop_solana_utils::get_key_pair_bytes_from_env_string_return_random_if_no_key_string_found(
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
pub fn setup_environment_and_get_configurations() -> ShopResult<ShopConfigurations> {
    let shop_configurations = get_environment_configurations()?;

    // setup our default logging format with levels according to .env
    env_logger::init();
    info!("environment setup complete");
    Ok(shop_configurations)
}

pub fn get_shop_state(shop_configurations: &ShopConfigurations) -> ShopResult<ShopState> {
    let program = shop_anchor_utils::try_get_program(shop_configurations)?;
    let shop_state = ShopState {
        shop_configurations: shop_configurations,
    };
    Ok(shop_state)
}