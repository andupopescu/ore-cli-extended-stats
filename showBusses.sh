#!/bin/bash


while true; do
    echo "Running ore busses..."
    start_time=$(date +%s%N)
    
    ./target/release/ore busses
    
    end_time=$(date +%s%N)
    duration=$(echo "scale=3; ($end_time - $start_time) / 1000" | bc)
    
    echo "----------------------------"
    printf "Command execution time: %.3f microseconds\n" $duration
    echo "----------------------------"
    sleep 1
done
