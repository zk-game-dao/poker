#!/bin/bash

set -euo pipefail

# === Configuration ===
export SNS_IDS_FILE="./sns_canister_ids.json"

# Load env vars (includes NETWORK, PEM_FILE, DEVELOPER_NEURON_ID, etc.)
source ./scripts/sns/setup_env.sh


# === Canisters to Register ===
export CANISTERS=(
  app_frontend
  btc_frontend
  cycle_dispenser
  log_store
  table_index
  tournament_index
  users_index
)

# === Validate Files ===
if [ ! -f "$PEM_FILE" ]; then
  echo "❌ PEM file not found at: $PEM_FILE"
  exit 1
fi

if [ ! -f "$SNS_IDS_FILE" ]; then
  echo "❌ SNS canister IDs file not found: $SNS_IDS_FILE"
  exit 1
fi

# === Collect Principal IDs ===
echo "🔍 Resolving canister IDs..."
PRINCIPAL_VEC=""
for CANISTER in "${CANISTERS[@]}"; do
  CID=$(dfx canister id "$CANISTER")
  echo " - $CANISTER: $CID"
  PRINCIPAL_VEC="${PRINCIPAL_VEC}principal\"$CID\"; "
done

# Remove trailing semicolon and space
PRINCIPAL_VEC=${PRINCIPAL_VEC%; }

# === Build Proposal ===
PROPOSAL_TEXT="(record {
  title=\"Register zkGame dapp canisters with SNS\";
  url=\"https://zk.game/whitepaper\";
  summary=\"This proposal registers the zkGame dapp canisters with the SNS to enable decentralized governance by the zkGame DAO.

zkGame DAO governs a suite of fully on-chain betting platforms built on the Internet Computer, including zkPoker and PurePoker. These platforms are designed with provable fairness, verifiable randomness, and a trustless architecture — addressing the transparency and security issues faced by traditional online betting systems.

Registering these canisters with the SNS ensures they are fully governed by the DAO, enabling the community to control upgrades, treasury management, and further development of the zkGame ecosystem, including upcoming initiatives like XDRC (a decentralized stablecoin), and new games such as on-chain chess and mahjong with wager elements.

For more information, visit https://zk.game/whitepaper\";
  action=opt variant {
    RegisterDappCanisters = record {
      canister_ids=vec { $PRINCIPAL_VEC }
    }
  }
})"

# === Generate Proposal File ===
echo "📝 Creating proposal..."
quill sns \
  --canister-ids-file "$SNS_IDS_FILE" \
  --pem-file "$PEM_FILE" \
  make-proposal \
  --proposal "$PROPOSAL_TEXT" \
  "$DEVELOPER_NEURON_ID" > register.json

# === Submit Proposal ===
echo "📤 Sending proposal..."
quill send register.json --insecure-local-dev-mode

echo "✅ Proposal submitted for ${#CANISTERS[@]} canisters."
