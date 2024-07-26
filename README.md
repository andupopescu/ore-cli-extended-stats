# Ore CLI
A command line interface for the Ore program.

## Don't do this
If you are not familiar with installing/configuring programs from source then this guide is not really for you. Use the packaged version listed in the main ore-cli repository.
https://github.com/regolith-labs/ore-cli

## Cloning the repositories
You will need to have git installed on your computer to clone, build and test this software. On debian/ubuntu this can usualy be done with ```sudo apt install git```. If this does not work, please google how to install git on your OS.

You will need to have build tools installed on your computer to enable compilation of some of the rust libraries. On debian/ubuntu this can usualy be done with ```sudo apt install build-essential```. Please google how to install development tools for your OS.

First create a suitable folder to clone the 3 git repositories to:
```sh
mkdir ~/ore2; cd ~/ore2
git clone https://github.com/regolith-labs/ore-cli
or if you want to use this version
git clone https://github.com/andupopescu/ore-cli-extended-stats.git
```
Execute each command separately one after the other watching for errors on the way.

## Building the utility
To build the Ore CLI, you will need to have the Rust programming language installed. You can install Rust by following the instructions on the [Rust website](https://www.rust-lang.org/tools/install).

Another prerequisite for these scripts is to install the solana cli from their [website](https://docs.solanalabs.com/cli/install)

The instructions presented here are for using a linux environment but also work on WSL2 on windows (I suspect they will work on most MAC computers as well).

Once you have Rust installed, you can build the Ore CLI by running the following command in the ore-cli folder:
```sh
cd ~/ore2/ore-cli-extended-stats
./build_and_mine.sh
```
The first build can be slow so please be patient while each library is compiled. Subsequent rebuilds will be significantly quicker. If the compilation fails, errors will be shown on screen for you to rectify.

The build process creates a compiled ore cli executable in the path ```./target/release/ore``` as well as a link to it in ```./ore```. This is the ore cli utility that you have compiled.

To test if the build was successful try running the command ```./ore``` and you should see some help from the ore-cli.

## Rebuilding & debugging the ore-cli utility
Save your edits to the source code then execute ```./build_and_mine.sh 1```. If the build is successful, a mining session will automatically be started for the first miner configured in ```ore_env.priv.sh```. Obviously, you need to follow the rest of the instructions here before attempting to do this as it does not know anything about your miner configuration yet.

## Setup your miner configuration
The scripts provided here all reference a file in the root of the ore-cli folder called ```ore_env.priv.sh```. This allows you to centralise your miner configuration and allows you to easily run as many miners as you have hardware to run them on and also to manage the wallets of your miners.

This file is excluded from the git repository as it contains personal information about your RPC URL, wallet locations, and a few other items about your miner configuration.

An example of this file is included in ```ore_env.priv.sh.sample``` and you can copy or rename this file to ```ore_env.priv.sh``` to get started. It has some comments in it that are probably worth reading.

You will need to configure at least 1 miner in this script to allow the other scripts in this application to work properly.

For each miner you need to specify RPC1, KEY1, THREADS1, PRIORITY_FEE1 and optionally MINER_WATTAGE_IDLE1 and MINER_WATTAGE_BUSY1.

A public RPC URL should work but I have generally found them to be quite unreliable for ORE mining. It is best to sign up for your own personal solana RPC endpoint from one of the providers such as QuickNode, Helius or any of the others.

A key file can be setup as described in the section ```Setting up a wallet```.

Threads should be set to a value less that or equal to the number of cores in your computer. Personally, I leave at least one thread free so the operating system can find time to respond whilst mining. eg. if you have 4 cores in your CPU then set threads to 3. This will lower your hashing power but means the computer does not grind to a halt for doing any other task whilst mining.

A priority fee is an extra cost that you can choose to append to a solana transaction to attempt to give your transaction more priority at your RPC server. Raising this can help you succeed in landing a transaction if the solana network is congested but comes with the side effect that EVERY transaction you use for meteor will have this additional cost attached. This can be left at 0 and should only be raised if you are continuously receiving submission errors whilst mining.

MINER_WATTAGE_IDLE1 is intended to be used to calculate energy consumption of your mining PC when it is not mining (idle).
MINER_WATTAGE_BUSY1 is intended to be used to calculate energy consumption of your mining PC when it is mining at the number of threads you intend to mine on (busy).
Both of these value can either be left to the defaults and ignore or you can use a watt meter to measure the power consumption of your PC's in both states. Hopefully, the stats page will reflect roughly how much electricity is costing for your mining session (see ELECTRICITY_COST_PER_KILOWATT_HOUR below).

There are 2 other global settings to configure:
COINGECKO_APIKEY: This will be used to lookup the ORE & SOL price from coingecko to convert the value of your wallet into dollars.
ELECTRICITY_COST_PER_KILOWATT_HOUR: This will be used to calculate the cost of electricity for each miner if the have a MINER_WATTAGE setting specified.

## Setting up a wallet
Each miner requires a unique wallet to mine to because of the staking mechanism. It is pointless to mine the same wallet on multiple miners. You can create a new wallet for use with ore-cli using the script below. Note that devnet wallet are not interchangeable with mainnet wallets and your RPC URL dictates what network the new wallet will be valid on.
```sh
mkdir ~/ore2/ore-cli/wallets
./createwallet.sh ~/ore2/ore-cli/wallets/wallet_devnet_test1.json
```
Note that this script will use the RPC1 URL defined in your configuration. This will lead you through creating a keypair file. It can be called whatever you like as long as you know where you create it and what it is called. It is best to keep these outside of the ore-cli folder so that it cannot accidentally be uploaded to git.

Remember and store your seed phrase in case you need to recreate it at a later date or import it into some other solana wallet app.

Once you have created the keypair file, you need to ensure the pathname is added as the KEY1 variable in ```ore_env.priv.sh```. This will be the wallet associated with miner 1.

## Funding your mining wallet
ORE mining is free. Your only charge for mining is the SOL transaction fees to submit your hashes each minute and also for staking/withdrawing your mined ORE.
Oh, and also your electric bill - you are taxing your computer harder than normal so it will be HOT, NOISY and cost more than normal to have powered on when mining.

You will need to transfer SOL into your mining wallet. Documenting this step is outwith the remit of this document but a pointer is to use something like the
Phantom Wallet browser plugin to transfer SOL from your main Solana wallet to your mining wallet.

If you are testing on devnet then you can airdrop yourself some SOL for free. The ```createwallet.sh``` script above will show you an exact command to do this
customised for your new wallet keypair file. It will be something like:
```sh
./airdropDevnetSol.sh 1 1.5
```
The command above will attempt to airdrop 1.5 SOL to miner 1's wallet. Be aware that your RPC will usually rate limit this and limit the actual amount you can airdrop and how often you can do it. You may need to try a few times perhaps decreasing the amount of SOL asked for. You could also try https://faucet.solana.com/ to airdrop your wallet address which is shown in the ```createwallet.sh``` output from the previous step.

ORE uses very little SOL every minute and it will cost around 0.000005 SOL * 60 mins * 24 hours = 0.0072 SOL for an entire day's mining. If 1 SOL costs \$200 then that is about \$1.44 per day per miner.

This calculation is assuming your transaction priority fee is 0. If you are submitting with a crazy high number then your costs can skyrocket quickly for each transaction but you should not need to use an higher number unless the Solana network is heavily congested.

## Manually starting a mining session
Execute the command:
```sh
./miner.sh 1
```
This will start up a miner process that will use the first wallet & RPC URL configured in the ```ore_env.priv.sh``` file. You will see the miner start up and watch it mine its first hash. After about 1 minute, you should get a transaction and a completed log message:
```sh
------------------------------------------------------------------------------------------------------------------------
Initialising: Miner 1
------------------------------------------------------------------------------------------------------------------------
Wallet: /home/paul/.config/solana/wallet_devnet_test1.json
RPC: https://XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX/
Priority fee: 0
Threads: 15
Buffer Time: 2
ore-cli: ./target/release/ore
------------------------------------------------------------------------------------------------------------------------
=======================================================================================================================================
| Rig Wattage When Idle: 15W
| Rig Wattage When Busy: 85W
| Cost of electric per kWHr: $0.4
| Wallet name: wallet_devnet_test1
=======================================================================================================================================
| Starting first pass... Miner 1...
=======================================================================================================================================
Pass 1 started at 23:06:24 on 2024-05-25                Mined for 0s    CPU: 45°C   0.25/0.64/0.56
        Currently Staked:   240.65112308950 ORE   Wallet:    5.479444 SOL    Last Withdrawal: 110.9 hours ago No Withdrawal Penalty
  [14s] Difficulty: 12 after 0 secs   Hashes: 11310   Hash: 1EYNw9yyteydXCE45hVR8EXVixxZBRwjcTm62m3mqqB
  [1s]  Attempt 1-6: SUCCESS                Txid: 3VJC3fZRxCwErdtg35TCL2nAY2eXykAqibWDVy617hGuV2SMrz9yBxFdhsfS4fyvrsDNnhTUNBzCTMkvms2Qxp6h
  [16s] Completed  Mined:     0.01100000000 ORE     Cost:   -0.000005 SOL    Session:     0.00000000000 ORE       0.000005 SOL
```

Congratulations, you have mined your first ORE. Large wallets start with humble rewards...

The miner will keep looping indefinitely until your wallet runs out of SOL. After each pass, any ORE mined is added to the wallet's staked ORE. The amount you have staked increases your earnings in subsequent mining passes.

If you have run out of SOL, the miner will pause for 1 minute then check again to see if you have deposited more SOL. Once SOL is added, the miner will automatically start mining again. If not, it will wait indefinitely unitl SOL is available or you kill the miner process.

The difficulty of the hash your miner has resolved will determine how much ORE is rewarded to all miners that submit a hash at that difficulty level. You will receive your share of the total rewards for that difficulty. A higher difficulty level solved will get you a higher amount of ORE rewarded.

The ORE rewarded is variable each pass so you will not usually get the same amount even if you solve the same difficulty level. There is a highly complex algorithm that calculates the rewards for each difficulty but you will need an enormous brain to understand how it is computed and if you think it is wrong then tough luck as that is what you are getting rewarded whether you like it or not. The idiots guide is the more ore you have staked, the longer it has been staked & how many other people submitted hashes at all difficulty levels all alter the payout structure. It is widely believed that the phase of the moon, the colour of the led on your mouse and your pets first name are all taken into account when calculating your reward.

The miner will keep track of your ORE mined, SOL spent and hashes calculated for your mining session.

At regular intervals, you will get a summary page detailing the progress of your mining session.
```sh
=======================================================================================================================================
|       Current ORE Price:            499.17 USD                Current SOL Price: $172.25 USD
|      Max session reward:     0.33260961607 ORE  ($166.03) at difficulty 18 during pass 2      [~33.2610% of supply]
|          Average reward:     0.09181490399 ORE  ($45.8312) over 5 passes                      [~9.1815% of supply]
|         Session Summary:            Profit                      Cost        Cost (Cloud)
|                  Tokens:     0.45907451993 ORE              0.000038 SOL    0.083 hours @ $0.16/hour
|              In dollars:            229.16 USD                  0.01 USD    0.01 USD
|          Profitablility:            229.14 USD
| Total Hashes in session: 0.2M         Average Hashes per pass: 45980          Threads: 8
|
| Difficulties solved during 5 passes:
|------------|----|----|----|----|
| Difficulty |  13|  15|  16|  18|
| Solves     |   1|   1|   2|   1|
| Percentage | 20%| 20%| 40%| 20%|
| Cumulative | 20%| 40%| 80%|100%|
=======================================================================================================================================
```
You are shown the current ORE and SOL prices in dollars if you have setup the coingecko api key in your config. See https://www.coingecko.com/en/api and look for the demo account option which is free.

You are presented with your maximum session reward gained in one pass and how much that is worth in dollars.

You are also shown your average amount of ORE earned per mining pass.

It then summarises your profit & costs for the session in tokens & dollars and give a profitability amount for this miner. Long may the rewards stay as high as they are currently on devnet. We will all be rich beyond our wildest dreams.

It will report how many hashes you have undertaken inthe session and provide an average number or hashes per minute. This can be used to estimate how powerful your miner is whilst perfoming actual ORE proof of work.

The difficulty table details how many of each difficulty level you have mined over the course of the session. Note that the same hardware may get a range of difficulties returned giving you a spread of results. Sometimes you get lucky and solve a more complex one in the 1 minute allowed and get a better reward! Over time the spread will gravitate to 2 or 3 difficulty levels which this miner can achieve. Comparing this table and the average hash rate for different computers/miners may lead you to decide to mine on your fastest or your most efficient. It's up to you to decide. You may decide to lower your threads to see if it adversely affect your spread of results.

You can safely stop the miner at any time without losing any staked rewards apart from the last pass you are mining when you stop the miner. On most computers this can be accomplished by pressing CTRL+C in the terminal where the miner is running. The next time you start your miner with the same wallet you will see that your staked ORE is preserved between mining sessions.

## Checking your Wallet Balance
You do not need to have a mining session running to see the wallet balances. You can check on the state of a wallet at any time by:
```sh
./walletBalance.sh 1
```
This will show the amount of unstaked and staked ORE for the particular miner number as specified in your ```ore_env.priv.sh```.
In the above example this would use the key specified by KEY1 for miner 1.

## Staking Additional ORE
If you have unstaked ORE stored in your wallet then you can opt to stake it to increase your rewards multiplier when mining with that key file.
You can add staked ore at any time (even whilst mining). To stake ORE, execute the following command:
```sh
./stakeOre.sh 1 all
./stakeOre.sh 1 2
```
The first example will stake ALL ore in wallet 1.
The second example will stake an additional 2 ORE in wallet 1

## Withdrawing Staked ORE
**Please be careful when staking ore - there is a penalty if you unstake it within 24 hours. You could lose part of your staked ORE if you withdraw too early. 24 hours after staking will return the entire staked amount to your wallet.**

You can withdraw your staked ORE at any point and move it to your wallet as ORE. This can then be transferred to another wallet or converted to another token (eg. to USDC or SOL).
```sh
./withdrawStakedOre.sh 1 all
./withdrawStakedOre.sh 1 15
```
Example 1 will unstaked all your staked ORE in wallet 1.
Example 2 will unstake 15 ORE from wallet 1 (if it has 15 ORE or more staked)

If you are trying to unstake too soon after mining or manually staking ORE then you will receive a warning and be told how much you ORE will permanently lose. You can opt out at this point and the ORE will all be left staked.
```sh
paul@paulsExtWin10:~/ore2/ore-cli$ ./withdrawStakedOre.sh 2 0.00189869703
20240519223218 wallet_devnet2.json Wallet 2 ORE balance: 0.00000000000 ORE ($0.00)      Staked: 0.10189869705 ORE ($28.77)
This wallet can currently withdraw up to 0.10189869705 staked ORE worth $28.77.
Your rewards of $28.77 are greater than $0.10 so proceeding to claim rewards.
----------------------------------------------------------------------------------------------------------

WARNING You are about to burn 0.00188107255 ORE!
Claiming more frequently than once per day is subject to a burn penalty.
Your last claim was 0.21 hours ago. You must wait 1426 minutes to avoid this penalty.

Are you sure you want to continue? [Y/n]
y
  [1s] (attempt 3) SUCCESS        Txid: 5mAbYMFNYET7k3PUY2SF6joJPip5MQ6DKEQgaDUEo4DBvGcnp8dtcriAQtvAocdxB3ixtt8T16ff4Woq7TgV1NR5                                                                                ==========================================================================================================
The wallet balance after withdrawing the staked ore is:
20240519223309 wallet_devnet2.json Wallet 2 ORE balance: 0.00001874524 ORE ($0.01)      Staked: 0.10000000002 ORE ($28.23)
```

## Close Accounts
TO DO - I have no idea what the purpose of this is yet so I cannot write this section yet.

## Monitoring Running Miners
A miner will continuously scroll text whilst it is mining. This can be hypnotic but can also be hard to read and analyse at a glance. Sometimes you just want to get a summary of where the miner is at. Each miner will create and update a log file in a folder called ./logs that details the statistics of the mining session as a whole and the details of the last mining pass.

The ```miner.sh``` script will automatically rotate these logs and keep up to 6 logfiles for each miner. This way you can compare results of previous mining sessions that are generally lost when the screen scrolls.

The name of the file will be the same name as your miner along with a number and a timestamp. So if you startup miner 1 with ```./miner.sh 1``` then the log file will be called ```./logs/Miner_1--1--XXXXXX.log```. This file is simply a text file that is continuosly overwritten when the miner is running. Being a text file, you can do whatever you like with it e.g. ```cat ./logs/Miner_1--1--*.log```. You could perhaps send this as an email, SMS message or possibly upload to a web site if you so desired.

There is a helper script called ```./watchStats.sh``` which accept the miner number as a parameter e.g. ```./watchStats.sh 1```. Open up a new terminal and start this script. When miner 1 is running, it will update every minute to show the stats for the miner. This can give you a single screen, non scrolling version of your miners logs. An example is below. 
```sh
Displaying log ./logs/Miner_1--1--2024-07-25-234208.log
=======================================================================================================================================
| Stats for Miner 1 pass 7 at 23:49:08 on 2024-07-25    [id]    Started at 23:42:09 on 2024-07-25
=======================================================================================================================================
|       Current ORE Price:            503.20 USD                Current SOL Price: $172.13 USD
|      Max session reward:     0.33260961607 ORE  ($166.03) at difficulty 18 during pass 2      [~33.2610% of supply]
|          Average reward:     0.07516858771 ORE  ($37.8248) over 7 passes                      [~7.5169% of supply]
|         Session Summary:            Profit                      Cost        Cost (Cloud)
|                  Tokens:     0.52618011395 ORE              0.000052 SOL    0.117 hours @ $0.16/hour
|              In dollars:            264.77 USD                  0.01 USD    0.02 USD
|          Profitablility:            264.75 USD
| Total Hashes in session: 0.3M         Average Hashes per pass: 46609          Threads: 8
|
| Difficulties solved during 7 passes:
|------------|----|----|----|----|----|
| Difficulty |  13|  15|  16|  17|  18|
| Solves     |   1|   1|   3|   1|   1|
| Percentage | 14%| 14%| 42%| 14%| 14%|
| Cumulative | 14%| 28%| 71%| 85%|100%|
=======================================================================================================================================
Pass 7 started at 23:48:07 on 2024-07-25                Mined for 5m 58s        CPU: 53.12/34.14/20.87
        Currently Staked:   418.98465585357 ORE   Wallet:    4.965540 SOL    
  [60s] Difficulty: 16 after 53 secs   Hashes: 47480   Hash: 11ZLR1N6Rk7zA4sksCvBrgwLaFdjuzocPaugDJa8HSM
  [61s] Completed  Mined:     0.03355288261 ORE     Cost:   -0.000007 SOL    Session:     0.52618011395 ORE       0.000052 SOL


```

You can also view the results of previous mining sessions by adding an extra parameter: ```./watchStats.sh 1 2``` will you you the final stats of the previous mining session allowing you to compare results. You can take the second parameter up to 6 ie 5 previous mining sessions.