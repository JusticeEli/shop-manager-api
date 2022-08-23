
use super::*;
pub fn try_get_program(shop_configurations: &ShopConfigurations) -> ShopResult<Program> {
    let program_id = try_get_program_id(&shop_configurations.program_id)?;

    let cluster = get_cluster(shop_configurations);
    let payer_key_pair = keypair_from_bytes(&shop_configurations.payer_key_pair_bytes)?;
    let client = configure_and_get_client(cluster, payer_key_pair);

    let program = client.program(program_id);
    return Ok(program);
}

pub fn configure_and_get_client(cluster: Cluster, payer_key_pair: Keypair) -> Client {
    let client = Client::new_with_options(
        cluster,
        Rc::new(payer_key_pair),
        CommitmentConfig::processed(),
    );
    client
}

pub fn get_cluster(shop_configurations: &ShopConfigurations) -> Cluster {
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