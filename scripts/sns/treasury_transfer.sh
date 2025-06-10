#!/usr/bin/env bash

set -euo pipefail

# ----------------------------------------------
# Treasury Transfer Script
#
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

# Load environment variables (sets NETWORK, PEM_FILE, DEVELOPER_NEURON_ID, etc.)
./scripts/sns/setup_env.sh

# Create the treasury transfer proposal
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
case "$ENV" in
  prod)
    quill send \
      --yes msg.json
    ;;
  *)
    quill send \
      --insecure-local-dev-mode \
      --yes msg.json
    ;;
esac

echo "✅ Treasury transfer proposal submitted successfully."
