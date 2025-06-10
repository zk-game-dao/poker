#!/usr/bin/env bash

set -euo pipefail

# ----------------------------------------------
# Usage:
#   ./scripts/sns/treasury_transfer.sh <to_principal> <amount_e8s> [env] [memo] [message]
#
# Parameters:
#   <to_principal> - Principal to receive the funds (required)
#   <amount_e8s>   - Amount in e8s (required)
#   [env]          - Environment: 'prod' or 'local' (default: 'local')
#   [memo]         - Optional: memo (default: 0)
#   [message]      - Optional: proposal message / title
#
# Example (local):
#   ./scripts/sns/treasury_transfer.sh abcde-principal 100000000
#
# Example (prod, with memo and message):
#   ./scripts/sns/treasury_transfer.sh abcde-principal 100000000 prod 123456 "Send 1 ICP to developer"
# ----------------------------------------------

if [[ $# -lt 2 ]]; then
  echo "Usage: $0 <to_principal> <amount_e8s> [env] [memo] [message]"
  exit 1
fi

TO_PRINCIPAL="$1"
AMOUNT="$2"
export ENV="${3:-local}"
MEMO="${4:-0}"
MESSAGE="${5:-"Treasury transfer to ${TO_PRINCIPAL}"}"

# Load env vars (includes NETWORK, PEM_FILE, DEVELOPER_NEURON_ID, etc.)
./scripts/sns/setup_env.sh

# Generate the proposal
quill sns \
  --canister-ids-file "${REPODIR}/sns_canister_ids.json" \
  --pem-file "${PEM_FILE}" \
  make-treasury-transfer-proposal \
  --amount-e8s "${AMOUNT}" \
  --to-principal "${TO_PRINCIPAL}" \
  --memo "${MEMO}" \
  --proposal-title "${MESSAGE}" \
  "${DEVELOPER_NEURON_ID}" > msg.json

# Submit the proposal
quill send \
  --insecure-local-dev-mode \
  --yes msg.json | grep -v "new_canister_wasm"

echo "Treasury transfer proposal submitted successfully"
