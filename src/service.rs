use std::{sync::Arc, time::Instant};
use std::sync::atomic::{AtomicBool, Ordering};
use warp::Filter;
use serde::{Deserialize, Serialize};
use drillx::{
    equix::{self},
    Hash,
};
use solana_rpc_client::spinner;
use hex;
use tokio::sync::Mutex as TokioMutex;

#[derive(Deserialize)]
struct MiningRequest {
    challenge: String, // Challenge string
    cutoff_time: u64,
    threads: u64,
    min_difficulty: u32,
    start_nonce: u64,
    end_nonce: u64,
}

#[derive(Serialize, Deserialize)]
pub struct MiningResponse {
    pub best_nonce: u64,
    pub best_difficulty: u32,
    pub best_hash: String,
    pub best_hash_bytes: Vec<u8>,
    pub url: String,
}

struct MiningState {
    stop_flag: Arc<AtomicBool>,
    is_mining: bool,
}

pub async fn start_service() {
    let mining_state = Arc::new(TokioMutex::new(MiningState {
        stop_flag: Arc::new(AtomicBool::new(false)),
        is_mining: false,
    }));

    let mine_route = warp::post()
        .and(warp::path("mine"))
        .and(warp::body::json())
        .and(warp::any().map(move || mining_state.clone()))
        .and_then(handle_mining_request);

    warp::serve(mine_route).run(([0, 0, 0, 0], 3030)).await;
}

async fn handle_mining_request(req: MiningRequest, mining_state: Arc<TokioMutex<MiningState>>) -> Result<impl warp::Reply, warp::Rejection> {
    let mut state = mining_state.lock().await;
    
    if state.is_mining {
        // Stop the existing mining operation
        state.stop_flag.store(true, Ordering::SeqCst);
        // Wait a bit for the threads to stop
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }

    // Reset the stop flag and set is_mining to true
    state.stop_flag.store(false, Ordering::SeqCst);
    state.is_mining = true;

    // Clone the stop_flag for use in the mining function
    let stop_flag = state.stop_flag.clone();

    // Release the lock before starting the mining operation
    drop(state);

    let challenge_vec = hex::decode(&req.challenge).map_err(|_| warp::reject::custom(InvalidChallenge))?;
    if challenge_vec.len() != 32 {
        return Err(warp::reject::custom(InvalidChallenge));
    }
    let mut challenge = [0u8; 32];
    challenge.copy_from_slice(&challenge_vec);


    //log req details
    println!("Challenge: {:?}", challenge);
    println!("Cutoff time: {}", req.cutoff_time);
    println!("Threads: {}", req.threads);
    println!("Min difficulty: {}", req.min_difficulty);
    println!("Start nonce: {}", req.start_nonce);
    println!("End nonce: {}", req.end_nonce);


    let solution = find_hash_par(
        challenge,
        req.cutoff_time,
        req.threads,
        req.min_difficulty,
        req.start_nonce,
        req.end_nonce,
        stop_flag,
    )
    .await;

    // Set is_mining back to false
    mining_state.lock().await.is_mining = false;

    Ok(warp::reply::json(&solution))
}

async fn find_hash_par(
    challenge: [u8; 32],
    cutoff_time: u64,
    threads: u64,
    min_difficulty: u32,
    start_nonce: u64,
    end_nonce: u64,
    stop_flag: Arc<AtomicBool>,
) -> MiningResponse {
    let progress_bar = Arc::new(spinner::new_progress_bar());
    progress_bar.set_message("Mining...");
    let handles: Vec<_> = (0..threads)
        .map(|i| {
            std::thread::spawn({
                let challenge = challenge.clone();
                let progress_bar = progress_bar.clone();
                let stop_flag = stop_flag.clone();
                let mut memory = equix::SolverMemory::new();
                move || {
                    let timer = Instant::now();
                    let range_size = (end_nonce - start_nonce) / threads;
                    let thread_start_nonce = start_nonce + range_size * i;
                    let thread_end_nonce = thread_start_nonce + range_size;
                    let mut best_nonce = thread_start_nonce;
                    let mut best_difficulty = 0;
                    let mut best_hash = Hash::default();
                    for nonce in thread_start_nonce..thread_end_nonce {
                        if stop_flag.load(Ordering::SeqCst) {
                            break;
                        }

                        if let Ok(hx) = drillx::hash_with_memory(
                            &mut memory,
                            &challenge,
                            &nonce.to_le_bytes(),
                        ) {
                            let difficulty = hx.difficulty();
                            if difficulty.gt(&best_difficulty) {
                                best_nonce = nonce;
                                best_difficulty = difficulty;
                                best_hash = hx;
                            }
                        }

                        if nonce % 100 == 0 {
                            if timer.elapsed().as_secs().ge(&cutoff_time) {
                                if best_difficulty.ge(&min_difficulty) {
                                    break;
                                }
                            } else if i == 0 {
                                progress_bar.set_message(format!(
                                    "Mining... ({} sec remaining)",
                                    cutoff_time.saturating_sub(timer.elapsed().as_secs()),
                                ));
                            }
                        }
                    }

                    (best_nonce, best_difficulty, best_hash)
                }
            })
        })
        .collect();

    let mut best_nonce = 0;
    let mut best_difficulty = 0;
    let mut best_hash = Hash::default();
    for h in handles {
        if let Ok((nonce, difficulty, hash)) = h.join() {
            if difficulty > best_difficulty {
                best_difficulty = difficulty;
                best_nonce = nonce;
                best_hash = hash;
            }
        }
    }

    progress_bar.finish_with_message(format!(
        "Best hash: {} (difficulty: {})",
        bs58::encode(best_hash.h).into_string(),
        best_difficulty
    ));

    MiningResponse {
        best_nonce,
        best_difficulty,
        best_hash: bs58::encode(&best_hash.h).into_string(),
        best_hash_bytes: best_hash.d.to_vec(),
        url: "https://equix.io".to_string(),
    }
}

#[derive(Debug)]
struct InvalidChallenge;
impl warp::reject::Reject for InvalidChallenge {}