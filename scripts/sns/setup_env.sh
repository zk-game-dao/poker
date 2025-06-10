#!/usr/bin/env bash

set -euo pipefail

# Fallback ENV
ENV="${ENV:-local}"

export DX_IDENT="dao-dev"

# Configure values based on ENV
case "$ENV" in
  prod)
    export NETWORK="https://icp-api.io"
    export DX_NETWORK="ic"
    export IC_URL="$NETWORK"
    ;;
  local-sns-testing)
    export NETWORK="http://0.0.0.0:8080"
    export DX_NETWORK="local"
    export IC_URL="$NETWORK"
    ;;
  local)
    export NETWORK="local"
    export DX_NETWORK="local"
    export IC_URL="http://127.0.0.1:4943"
    ;;
  *)
    echo "Unknown ENV: $ENV"
    exit 1
    ;;
esac

# Use current working directory as base
export REPODIR="$(pwd)"

# Use identity
dfx identity use "$DX_IDENT"

# Optional inputs (may be set by script before calling this)
export NAME="${NAME:-test}"
export WASM="${WASM:-}"
export ARG="${ARG:-()}"

# Get principal and PEM
export DX_PRINCIPAL="$(dfx identity get-principal)"
export DX_VERSION="$(dfx --version | sed 's/^dfx //')"
export PEM_FILE="$(readlink -f ~/.config/dfx/identity/${DX_IDENT}/identity.pem)"

# Ensure PEM file exists
dfx identity export "$DX_IDENT" > "$PEM_FILE"

# Generate sns_canister_ids.json
./scripts/sns/build_sns_canister_ids.sh > sns_canister_ids.json

# Load SNS canister IDs
if [[ -f "${REPODIR}/sns_canister_ids.json" ]]; then
  export SNS_GOVERNANCE_CANISTER_ID=$(jq -r '.governance_canister_id' "${REPODIR}/sns_canister_ids.json")
  export SNS_INDEX_CANISTER_ID=$(jq -r '.index_canister_id' "${REPODIR}/sns_canister_ids.json")
  export SNS_LEDGER_CANISTER_ID=$(jq -r '.ledger_canister_id' "${REPODIR}/sns_canister_ids.json")
  export SNS_ROOT_CANISTER_ID=$(jq -r '.root_canister_id' "${REPODIR}/sns_canister_ids.json")
  export SNS_SWAP_CANISTER_ID=$(jq -r '.swap_canister_id' "${REPODIR}/sns_canister_ids.json")
fi

# Fetch neuron ID for current principal
export DEVELOPER_NEURON_ID="$(dfx canister \
  --network "${NETWORK}" \
  call "${SNS_GOVERNANCE_CANISTER_ID}" \
  list_neurons "(record {of_principal = opt principal\"${DX_PRINCIPAL}\"; limit = 1})" \
  | idl2json \
  | jq -r '.neurons[0].id[0].id' \
  | python3 -c 'import sys; ints=sys.stdin.readlines(); sys.stdout.write(bytearray(eval("".join(ints))).hex())')"
