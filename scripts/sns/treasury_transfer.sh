#!/usr/bin/env bash

set -euo pipefail

# ----------------------------------------------
# Treasury Transfer Script (Fixed)
#
# Usage:
#   ./scripts/sns/treasury_transfer.sh <to_principal> <amount_e8s> [env] [message] [memo]
#
# Parameters:
#   <to_principal> - Principal to receive the funds (required)
#   <amount_e8s>   - Amount in e8s (required)
#   [env]          - Environment: 'prod' or 'local' (default: 'local')
#   [message]      - Optional: proposal title (default based on principal)
#   [memo]         - Optional: memo (default: null)
# ----------------------------------------------

if [[ $# -lt 2 ]]; then
  echo "Usage: $0 <to_principal> <amount_e8s> [env] [message] [memo]"
  exit 1
fi

TO_PRINCIPAL="$1"
AMOUNT_E8s="$2"
ENV="${3:-local}"
MESSAGE="${4:-"Treasury transfer to ${TO_PRINCIPAL}"}"
MEMO="${5:-null}"

source ./scripts/sns/setup_env.sh

SUMMARY="This proposal transfers ${AMOUNT_E8s} e8s from the SNS treasury to principal ${TO_PRINCIPAL}."

PROPOSAL="(record {
  title=\"${MESSAGE}\";
  url=\"https://zk.game\";
  summary=\"${SUMMARY}\";
  action=opt variant {
    TransferSnsTreasuryFunds = record {
      from_treasury = 1 : int32;
      amount_e8s = ${AMOUNT_E8s} : nat64;
      to_principal = opt principal \"${TO_PRINCIPAL}\";
      memo = ${MEMO};
      to_subaccount = null;
    }
  }
})"

echo "$PROPOSAL"

quill sns \
  --canister-ids-file "${REPODIR}/sns_canister_ids.json" \
  --pem-file "${PEM_FILE}" \
  make-proposal \
  --proposal "${PROPOSAL}" \
  "${DEVELOPER_NEURON_ID}" > msg.json

case "$ENV" in
  prod)
    quill send --yes msg.json
    ;;
  *)
    quill send --insecure-local-dev-mode --yes msg.json
    ;;
esac

echo "✅ Treasury transfer proposal submitted successfully."
