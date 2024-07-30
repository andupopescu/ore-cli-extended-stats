use colored::Colorize;
use crate::{
	utils::{amount_u64_to_string, amount_u64_to_f64, get_config},
	Miner,
};

impl Miner {
    pub async fn rewards(&self) {
        let config = get_config(&self.rpc_client).await;
        let base_reward_rate = config.base_reward_rate;
        let base_difficulty = config.min_difficulty;

        let mut s = format!(
            "Base Reward Rate: {}: {} ORE",
            base_difficulty,
            amount_u64_to_string(base_reward_rate)
        )
        .to_string();
        let mut diff_to_target=0;
        for i in 1..32 as u64 {
            let reward_rate = base_reward_rate.saturating_mul(2u64.saturating_pow(i as u32));
                if amount_u64_to_f64(reward_rate)>1.0 && diff_to_target==0 {
                    diff_to_target = base_difficulty + i;
                }
            s = format!(
                "{}\n{}: {} ORE",
                s,
                base_difficulty + i,
                amount_u64_to_string(reward_rate)
            );
        }
        println!("{}", s);

		println!("You should target difficulty {}+ to get max 1 ORE rewarded", diff_to_target.to_string().green());
    }
}
