#!/usr/bin/env bash

set -euo pipefail

# ----------------------------------------------
# Usage:
#   ./scripts/sns/upgrade.sh <name> [arg] [env]
#
# Parameters:
#   <name> - Canister name to upgrade (required)
#   [arg]  - Upgrade argument or path to argument file (default: '(record {})')
#   [env]  - Environment: 'prod' or 'local' (default: 'local')
#
# Example (local):
#   ./scripts/sns/upgrade.sh governance
#
# Example (file arg):
#   ./scripts/sns/upgrade.sh governance didc_args.txt
#
# Example (prod):
#   ./scripts/sns/upgrade.sh governance '(record {})' prod
# ----------------------------------------------

if [[ $# -lt 1 ]]; then
  echo "Usage: $0 <name> [arg] [env]"
  exit 1
fi

export NAME="$1"
export ARG="${2:-'(record {})'}"
export ENV="${3:-local}"

# Load secrets and common env (includes NETWORK, DX_NETWORK, PEM_FILE, etc.)
./scripts/sns/setup_env.sh

# Determine if ARG is a file path or raw value
if [ -f "${ARG}" ]; then
  ARGFLAG="--canister-upgrade-arg-path"
else
  ARGFLAG="--canister-upgrade-arg"
fi

# Build and compress WASM if not already set
if [[ -z "${WASM:-}" ]]; then
  WASM=".dfx/${DX_NETWORK}/canisters/${NAME}/${NAME}"
  rm -f "${WASM}-s.wasm.gz"
  dfx build --network "${NETWORK}" "${NAME}"
  ic-wasm "${WASM}.wasm" -o "${WASM}-s.wasm" shrink
  gzip "${WASM}-s.wasm"
  export WASM="${WASM}-s.wasm.gz"
fi

# Fetch the canister ID
CID="$(dfx canister --network "${NETWORK}" id "${NAME}")"

# Create the proposal message
quill sns \
  --canister-ids-file "${REPODIR}/sns_canister_ids.json" \
  --pem-file "${PEM_FILE}" \
  make-upgrade-canister-proposal \
  --target-canister-id "${CID}" \
  --mode upgrade \
  --wasm-path "${WASM}" \
  "${ARGFLAG}" "${ARG}" \
  "${DEVELOPER_NEURON_ID}" > msg.json

# Submit the proposal#!/usr/bin/env bash

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
case "$ENV" in
  prod)
    quill send \
      --yes msg.json \
      | grep -v "new_canister_wasm"
    ;;
  *)
    quill send \
      --insecure-local-dev-mode \
      --yes msg.json \
      | grep -v "new_canister_wasm"
    ;;
esac

echo "Success"
