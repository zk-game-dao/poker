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

# Submit the proposal
quill send \
  --insecure-local-dev-mode \
  --yes msg.json | grep -v "new_canister_wasm"

echo "Success"
