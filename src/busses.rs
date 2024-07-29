use ore_api::{
    consts::{BUS_ADDRESSES, TOKEN_DECIMALS},
    state::Bus,
};
use ore_utils::AccountDeserialize;

use crate::Miner;

use solana_program::pubkey::Pubkey;

impl Miner {
    pub async fn busses(&self) -> (Pubkey, f64) {
        let client = self.rpc_client.clone();
        let mut max_rewards = 0.0;
        let mut max_bus = BUS_ADDRESSES[0];

        for address in BUS_ADDRESSES.iter() {
            let data = client.get_account_data(address).await.unwrap();
            match Bus::try_from_bytes(&data) {
                Ok(bus) => {
                    let rewards = (bus.rewards as f64) / 10f64.powf(TOKEN_DECIMALS as f64);
                    println!("Bus {}: {:} ORE", bus.id, rewards);
                    if rewards > max_rewards {
                        max_rewards = rewards;
                        max_bus = *address;
                    }
                }
                Err(_) => {}
            }
        }

        (max_bus, max_rewards)
    }


    pub async fn get_best_bus(&self) -> Pubkey {
        let client = self.rpc_client.clone();
        let mut max_rewards = 0.0;
        let mut max_bus = BUS_ADDRESSES[0];

        for address in BUS_ADDRESSES.iter() {
            let data = client.get_account_data(address).await.unwrap();
            if let Ok(bus) = Bus::try_from_bytes(&data) {
                let rewards = (bus.rewards as f64) / 10f64.powf(TOKEN_DECIMALS as f64);
                if rewards > max_rewards {
                    max_rewards = rewards;
                    max_bus = *address;
                }
            }
        }

        max_bus
    }

}