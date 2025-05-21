#!/bin/bash

# Array of canister IDs - replace the placeholder with your actual canister IDs
CANISTER_IDS=(
    c3l5a-iqaaa-aaaam-qb6qa-cai
)

# Amount of cycles to top up for each canister
CYCLES_AMOUNT="300000000000"

# Network to use
NETWORK="ic"

# Function to top up a canister
top_up_canister() {
    local canister_id=$1
    echo "Topping up canister: $canister_id with $CYCLES_AMOUNT cycles..."
    dfx cycles top-up $canister_id $CYCLES_AMOUNT --network $NETWORK
    
    # Check if the command was successful
    if [ $? -eq 0 ]; then
        echo "✅ Successfully topped up canister: $canister_id"
    else
        echo "❌ Failed to top up canister: $canister_id"
    fi
    echo "-----------------------------------------"
}

# Main script execution
echo "Starting canister top-up process..."
echo "-----------------------------------------"

# Loop through each canister ID and top it up
for canister_id in "${CANISTER_IDS[@]}"; do
    top_up_canister $canister_id
done

echo "Completed canister top-up process!"
