#!/bin/bash

# set -x

DFX_NETWORK=${DFX_NETWORK:-local}

echo "Using DFX network: $DFX_NETWORK"
NETWORK_ARG="--network $DFX_NETWORK"
if [ "$DFX_NETWORK" == "local" ]; then
    NETWORK_ARG=""
fi

./scripts/download-sign-in-providers.sh

dfx canister create ic_siwb_provider $NETWORK_ARG
# Write the argument into a var

domain="hffks-yiaaa-aaaah-qqana-cai.icp0.io"
uri="https://hffks-yiaaa-aaaah-qqana-cai.icp0.io"

if [ "$DFX_NETWORK" == "local" ]; then
    domain="127.0.0.1";
    uri="http://127.0.0.1:5173";
fi

argument='(
    record {
        domain = "'"$domain"'";
        uri = "'"$uri"'";
        salt = "V8oaXyWK3s1E85Gsxb8GF6N29cKjBmcZ";
        statement = opt "Login to Pure Poker";
    }
)'

echo "Deploying ic_siwb_provider canister..."
dfx deploy ic_siwb_provider $NETWORK_ARG --argument "$argument"
