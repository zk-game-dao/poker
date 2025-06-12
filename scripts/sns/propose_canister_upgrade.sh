#!/usr/bin/env bash

set -euo pipefail

# ----------------------------------------------
# Usage:
#   ./scripts/sns/upgrade.sh <name> [env] [arg]
#
# Parameters:
#   <name> - Canister name to upgrade (required)
#   [env]  - Environment: 'prod' or 'local' (default: 'local')
#   [arg]  - Upgrade argument or path to argument file (default: '(record {})')
#
# Example (local):
#   ./scripts/sns/upgrade.sh governance
#
# Example (file arg):
#   ./scripts/sns/upgrade.sh governance didc_args.txt
#
# Example (prod):
#   ./scripts/sns/upgrade.sh governance prod '(record {})'
# ----------------------------------------------

if [[ $# -lt 1 ]]; then
  echo "Usage: $0 <name> [arg] [env]"
  exit 1
fi

export NAME="$1"
export ENV="${2:-local}"
export ARG="${3:-"(record {})"}"

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

# Optional inputs (may be set by script before calling this)
export NAME="${NAME:-test}"
export WASM="${WASM:-}"

# Build and use compressed WASM directly
if [[ -z "${WASM:-}" ]]; then
  echo "Building wasm" 
  
  dfx build --network "${NETWORK}" "${NAME}"
  WASM=".dfx/${DX_NETWORK}/canisters/${NAME}/${NAME}.wasm.gz"

  if [[ ! -f "${WASM}" ]]; then
    echo "ERROR: Built WASM not found at ${WASM}"
    exit 1
  fi

  export WASM="${WASM}"
fi

# Fetch the canister ID
CID="$(dfx canister --network "${NETWORK}" id "${NAME}")"

if [[ -z "${DEVELOPER_NEURON_ID:-}" ]]; then
  echo "❌ ERROR: DEVELOPER_NEURON_ID is not set"
  exit 1
fi

echo "Creating proposal message"
quill sns \
  --canister-ids-file "${REPODIR}/sns_canister_ids.json" \
  --pem-file "${PEM_FILE}" \
  make-upgrade-canister-proposal \
  --target-canister-id "${CID}" \
  --mode upgrade \
  --wasm-path "${WASM}" \
  "${ARGFLAG}" "${ARG}" \
  --title "Upgrade ${NAME} to new version" \
  --summary "Upgrading ${NAME} with newly built wasm" \
  --url "https://zk.game/" \
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
