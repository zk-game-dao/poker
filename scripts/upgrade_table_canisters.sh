#!/bin/bash

# Check current identity
echo "Current dfx identity:"
current_identity=$(dfx identity whoami)
echo "$current_identity"
echo ""

# Confirm identity and upgrade intention
read -p "Are you sure you want to upgrade canisters using identity '$current_identity'? (yes/no): " confirm

if [ "$confirm" != "yes" ]; then
    echo "Canister upgrade aborted."
    exit 1
fi

# Define an array of your canister IDs
TABLE_CANISTER_IDS=(
    "gig5c-vaaaa-aaaah-qpvjq-cai"
    "zal4c-6aaaa-aaaah-qpxdq-cai"
    "b4hyc-miaaa-aaaah-qpv2a-cai"
    "gph3w-yyaaa-aaaah-qpvja-cai"
    "bjajp-naaaa-aaaah-qpvzq-cai"
)

# Path to your compiled Wasm module
WASM_PATH="target/wasm32-unknown-unknown/release/table_canister.wasm"

# Check if WASM file exists
if [ ! -f "$WASM_PATH" ]; then
    echo "Error: WASM file not found at $WASM_PATH"
    exit 1
fi

echo "About to upgrade ${#TABLE_CANISTER_IDS[@]} canisters with wasm from: $WASM_PATH"
echo ""
read -p "Proceed with upgrade? (yes/no): " upgrade_confirm

if [ "$upgrade_confirm" != "yes" ]; then
    echo "Canister upgrade aborted."
    exit 1
fi

# Counter for tracking progress
total_canisters=${#TABLE_CANISTER_IDS[@]}
current_canister=0
successful_upgrades=0
failed_upgrades=0

# Loop over each canister ID and perform the upgrade
for CANISTER_ID in "${TABLE_CANISTER_IDS[@]}"
do
    ((current_canister++))
    echo "[$current_canister/$total_canisters] Upgrading canister: $CANISTER_ID"
    
    # Execute the dfx canister install command with the --mode upgrade option
    if dfx canister install "$CANISTER_ID" --mode upgrade --wasm "$WASM_PATH" --network ic; then
        echo "Successfully upgraded canister: $CANISTER_ID"
        ((successful_upgrades++))
    else
        echo "Failed to upgrade canister: $CANISTER_ID"
        ((failed_upgrades++))
    fi
    echo "--------------------------------------"
done

# Print summary
echo "Upgrade Summary:"
echo "Total canisters processed: $total_canisters"
echo "Successful upgrades: $successful_upgrades"
echo "Failed upgrades: $failed_upgrades"
