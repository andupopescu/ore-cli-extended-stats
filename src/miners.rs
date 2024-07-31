use solana_transaction_status::UiTransactionEncoding;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use solana_sdk::native_token::lamports_to_sol;
use solana_transaction_status::option_serializer::OptionSerializer;
// use solana_transaction_status::EncodedTransaction;
// use solana_transaction_status::UiMessage;
use std::str::FromStr;
use chrono::prelude::*;
use std::string::String;
use colored::*;

use crate::{
    // send_and_confirm::ComputeBudget,
    Miner,
};

impl Miner {
    pub async fn miners(&self) {
        let client = self.rpc_client.clone();
        let ore_program_address = Pubkey::from_str("oreV2ZymfyeXgNgBdqMkumTqqAprVqgBWQfoYkrtKWQ").unwrap();
        println!("Getting signatures from {:?}", ore_program_address);
        let signatures = client
            .get_signatures_for_address(&ore_program_address)
            .await;

        let sigs = signatures.unwrap();

        let mut i = 0;
        let mut max_diff = 0;
        let mut cutoff_timestamp: i64 = Local::now().timestamp() - 60;
        for sig in sigs {
            let block_timestamp = sig.block_time.unwrap();
            if i == 0 {
                cutoff_timestamp = block_timestamp - 60;
            }
            if block_timestamp >= cutoff_timestamp {
                let the_sig = Signature::from_str(&sig.signature).unwrap();
                match client.get_transaction(&the_sig, UiTransactionEncoding::Json).await {
                    Ok(transaction_details) => {
                        let tx = transaction_details.transaction;
                        let m = tx.meta.clone().unwrap();
                        let fee = lamports_to_sol(m.fee);

                        // Extract the fee payer's address with error handling
                        let fee_payer = match &tx.transaction {
                            solana_transaction_status::EncodedTransaction::Json(json_tx) => {
                                match &json_tx.message {
                                    solana_transaction_status::UiMessage::Parsed(parsed_message) => {
                                        parsed_message.account_keys.get(0)
                                            .map(|key| key.pubkey.clone())
                                            .unwrap_or_else(|| "Unable to extract fee payer".to_string())
                                    },
                                    solana_transaction_status::UiMessage::Raw(raw_message) => {
                                        raw_message.account_keys.get(0)
                                            .cloned()
                                            .unwrap_or_else(|| "Unable to extract fee payer".to_string())
                                    },
                                }
                            },
                            _ => "Unable to extract fee payer".to_string(),
                        };

                        if let Some(meta) = tx.meta {
                            match meta.log_messages {
                                OptionSerializer::Some(logs) => {
                                    for log in logs {
                                        if log.contains("Diff ") {
                                            let diff: i32 = log.strip_prefix("Program log: Diff ").unwrap().parse().expect("Failed to parse string to integer");
                                            let timestamp = sig.block_time.unwrap();
                                            let datetime = Utc.timestamp_opt(timestamp as i64, 0).unwrap();
                                            let formatted_time = datetime.format("%Y-%m-%d %H:%M:%S").to_string();
                                            let short_sig = sig.signature.chars().take(19).collect::<String>();
                                            i += 1;

                                            println!("Difficulty: {}   {:03} {}   {:10.9} SOL  Wallet: {}  tx:{}...", 
                                                format!("{:02}", diff).to_string().green(), 
                                                i, 
                                                formatted_time.to_string().dimmed(), 
                                                fee, 
                                                fee_payer.dimmed(),
                                                short_sig.dimmed());
                                            if diff > max_diff {
                                                max_diff = diff;
                                            }
                                        }
                                    }
                                    if i >= 100 {
                                        break;
                                    }
                                },
                                OptionSerializer::None => {
                                    println!("{} {:?} - No logs available", i, sig.signature);
                                },
                                OptionSerializer::Skip => {
                                    println!("{} {:?} - Logs were skipped", i, sig.signature);
                                }
                            }
                        } else {
                            println!("{} {:?} - No metadata available", i, sig.signature);
                        }
                    }
                    Err(err) => eprintln!("{}", err)
                }
            }
        }

        println!("-------------------------------------------------------------");
        println!("Max difficulty for {} miners over last minute is {}", i.to_string().green().bold(), max_diff.to_string().green().bold());
    }
}