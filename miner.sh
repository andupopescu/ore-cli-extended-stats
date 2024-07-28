#!/bin/bash
#
#devnet config

if [ $# -ne 1 ]; then
	echo "USAGE: $0 [integer representing the miner number in ore_env.priv.sh]"
	exit 1
fi
source ./ore_env.sh $1

solana config set --url ${RPC1} >/dev/null

rotate_logs() {
	local log_file_base=$1
	for i in {5..1}; do
		if [ -f "${log_file_base}--${i}.log" ]; then
			mv "${log_file_base}--${i}.log" "${log_file_base}--$((i+1)).log"
		fi
		if [ -f "${log_file_base}--${i}.json" ]; then
			mv "${log_file_base}--${i}.json" "${log_file_base}--$((i+1)).json"
		fi
	done
}

remove_log_file() {
	local log_file_base=$1
	local index=$2
	rm -f "${log_file_base}--${index}.log"
	rm -f "${log_file_base}--${index}.json"
}

while true; do
	echo ------------------------------------------------------------------------------------------------------------------------
	echo Initialising:		${MINER_NAME}
	echo ------------------------------------------------------------------------------------------------------------------------
	echo Wallet:			${KEY}
	echo RPC:				${RPC_URL}
	echo Priority fee:		${FEE}
	echo Threads:			${THREADS}
	echo Buffer Time:		${BUFFER_TIME}
	echo ore-cli:			${ORE_BIN}

	# rotate any previous logs to keep last 6
	if [ ! -d "./logs" ]; then
		mkdir "./logs"
	fi
	STATS_LOGFILE_BASE="./logs/${MINER_NAME// /_}"
	remove_log_file "${STATS_LOGFILE_BASE}" 6
	rotate_logs "${STATS_LOGFILE_BASE}"
	remove_log_file "${STATS_LOGFILE_BASE}" 1
	STATS_LOGFILE="${STATS_LOGFILE_BASE}--1--$(date '+%Y-%m-%d-%H%M%S').log"
	STATS_JSONFILE="${STATS_LOGFILE_BASE}--1--$(date '+%Y-%m-%d-%H%M%S').json"
	export STATS_LOGFILE
	export STATS_JSONFILE

	export MINER_NAME
	WALLET_NAME=${KEY##*/}
	WALLET_NAME=${WALLET_NAME%.*}
	export WALLET_NAME
	export MINER_WATTAGE_IDLE
	export MINER_WATTAGE_BUSY
	export MINER_COST_PER_KILOWATT_HOUR 
	export MINER_DESIRED_DIFFICULTY_LEVEL 
	
	# start the miner
	COMMAND="${ORE_BIN} mine --rpc ${RPC_URL} --keypair ${KEY} --priority-fee=${FEE:-0} --threads ${THREADS:-1} --buffer-time ${BUFFER_TIME:-2}"
	# echo ${COMMAND}
	eval $COMMAND
	[ $? -eq 0 ] && break

	echo ------------------------------------------------------------------------------------------------------------------------
	echo `date +'%Y-%m-%d %H:%M:%S'` "Restarting miner process in 10 seconds..."
	sleep 10
done