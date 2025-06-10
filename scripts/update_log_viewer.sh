#!/bin/bash

# Array of canister IDs - replace the placeholders with your actual canister IDs
CANISTER_IDS=(
    z5wv2-oiaaa-aaaam-qbeyq-cai
    j5kch-7yaaa-aaaam-qds2a-cai
    zuv6g-yaaaa-aaaam-qbeza-cai
    j2let-saaaa-aaaam-qds2q-cai
    ztuys-vyaaa-aaaam-qbezq-cai
    zgtj7-uqaaa-aaaam-qbe2a-cai
    zbspl-ziaaa-aaaam-qbe2q-cai
    zjdqr-qiaaa-aaaam-qdfaa-cai
    zocwf-5qaaa-aaaam-qdfaq-cai
    laxmp-mqaaa-aaaam-qdsvq-cai
    lvq5c-nyaaa-aaaam-qdswa-cai
)

# Hardcoded log viewer principals
LOG_VIEWERS=(
    km7qz-4bai4-e5ptx-hgrck-z3web-ameqg-ksxcf-u7wbr-t5fna-i7bqp-hqe
    uyxh5-bi3za-gxbfs-op3gj-ere73-a6jhv-5jky3-zawef-b5r2s-k26un-sae
)

# Network to use
NETWORK="ic"

# Function to add log viewer
add_log_viewer() {
    local canister_id=$1
    local principal=$2
    
    echo "Adding log viewer $principal to canister: $canister_id..."
    dfx canister update-settings $canister_id --add-log-viewer $principal --network $NETWORK
    
    # Check if the command was successful
    if [ $? -eq 0 ]; then
        echo "✅ Successfully added $principal to $canister_id"
    else
        echo "❌ Failed to add $principal to $canister_id"
    fi
    echo "-----------------------------------------"
}

# Main script execution
echo "Starting log viewer addition process..."
echo "-----------------------------------------"

# Loop through each canister ID
for canister_id in "${CANISTER_IDS[@]}"; do
    # Add each log viewer for the current canister
    for viewer in "${LOG_VIEWERS[@]}"; do
        add_log_viewer $canister_id $viewer
    done
done

echo "Completed log viewer addition process!"
