#!/bin/bash

set -e

SNS_ROOT_ID=$(dfx canister id sns_root)
PRINCIPAL=$(dfx identity get-principal)

CANISTERS=$(jq -r '.canisters | to_entries[] | select(.value.remote == null) | .key' dfx.json)

for CANISTER in $CANISTERS; do
  echo "Checking controller status for $CANISTER..."
  CONTROLLERS=$(dfx canister info "$CANISTER" | grep "Controllers:" | cut -d':' -f2)

  if echo "$CONTROLLERS" | grep -q "$PRINCIPAL"; then
    echo "✅ You are a controller of $CANISTER. Updating settings..."
    dfx canister update-settings --add-controller "$SNS_ROOT_ID" "$CANISTER"
  else
    echo "⚠️ Skipping $CANISTER — current principal is not a controller."
  fi
done
