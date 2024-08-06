#!/bin/bash

while true; do
    echo "Running ore rewards..."
    start_time=$(date +%s%N)
    
    ./target/release/ore rewards
    
    end_time=$(date +%s%N)
    duration=$(echo "scale=3; ($end_time - $start_time) / 1000" | bc)
    
    echo "----------------------------"
    printf "Command execution time: %.3f microseconds\n" $duration
    echo "----------------------------"
    sleep 1
done