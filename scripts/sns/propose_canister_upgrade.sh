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
export ARG="${2:-"(record {})"}"
export ENV="${3:-local}"

# Load secrets and common env (includes NETWORK, DX_NETWORK, PEM_FILE, etc.)
source ./scripts/sns/setup_env.sh

if [[ "$ARG" =~ ^\' || "$ARG" =~ \'\$ ]]; then
  echo "ERROR: Argument appears to include literal single quotes. Use (record {}) instead of '(record {})'."
  exit 1
fi

# Determine if ARG is a file path or raw value
if [ -f "${ARG}" ]; then
  ARGFLAG="--canister-upgrade-arg-path"
else
  ARGFLAG="--canister-upgrade-arg"
fi

# Build and compress WASM if not already set
if [[ -z "${WASM:-}" ]]; then
  echo "Building wasm" 
  
  WASM=".dfx/${DX_NETWORK}/canisters/${NAME}/${NAME}"
  rm -f "${WASM}-s.wasm.gz"
  dfx build --network "${NETWORK}" "${NAME}"
  ic-wasm "${WASM}.wasm" -o "${WASM}-s.wasm" shrink
  gzip "${WASM}-s.wasm"
  export WASM="${WASM}-s.wasm.gz"
fi

# Fetch the canister ID
CID="$(dfx canister --network "${NETWORK}" id "${NAME}")"

echo "Creating proposal message"
quill sns \
  --canister-ids-file "${REPODIR}/sns_canister_ids.json" \
  --pem-file "${PEM_FILE}" \
  make-upgrade-canister-proposal \
  --target-canister-id "${CID}" \
  --mode upgrade \
  --wasm-path "${WASM}" \
  "${ARGFLAG}" "${ARG}" \
  "${DEVELOPER_NEURON_ID}" > msg.json

echo "Submitting the proposal..."
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

