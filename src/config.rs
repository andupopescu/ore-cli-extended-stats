use colored::Colorize;

use crate::{
    utils::{amount_u64_to_string, get_config},
    Miner,
};

impl Miner {
    pub async fn config(&self) {
        let config = get_config(&self.rpc_client).await;
        println!("{}: {}", "Last reset".bold(), config.last_reset_at);
        println!("{}: {} ORE", "Top balance".bold(), amount_u64_to_string(config.top_balance));
        println!("{}: {}", "Min difficulty".bold(), config.min_difficulty);
        println!("{}: {} ORE", "Base reward rate".bold(), amount_u64_to_string(config.base_reward_rate));
        println!(
            "{}: {} ORE",
            "Top stake".bold(),
            amount_u64_to_string(config.top_balance),
        );
    }
}
