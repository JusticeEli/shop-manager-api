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