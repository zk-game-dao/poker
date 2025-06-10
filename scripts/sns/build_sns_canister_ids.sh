# Copied from https://raw.githubusercontent.com/open-chat-labs/open-chat/2b7a455df9f4fa4b9523644c535b343fc170f374/sns/scripts/utils/build_sns_canister_ids_json.sh

# Read each SNS canister id using dfx
GOVERNANCE_CANISTER_ID=$(dfx -qq canister --network $NETWORK id sns_governance)
INDEX_CANISTER_ID=$(dfx -qq canister --network $NETWORK id sns_index)
LEDGER_CANISTER_ID=$(dfx -qq canister --network $NETWORK id sns_ledger)
ROOT_CANISTER_ID=$(dfx -qq canister --network $NETWORK id sns_root)
SWAP_CANISTER_ID=$(dfx -qq canister --network $NETWORK id sns_swap)

# Write the json to stdout
echo "{"
echo "  \"dapp_canister_id_list\": [],"
echo "  \"governance_canister_id\": \"$GOVERNANCE_CANISTER_ID\","
echo "  \"index_canister_id\": \"$INDEX_CANISTER_ID\","
echo "  \"ledger_canister_id\": \"$LEDGER_CANISTER_ID\","
echo "  \"root_canister_id\": \"$ROOT_CANISTER_ID\","
echo "  \"swap_canister_id\": \"$SWAP_CANISTER_ID\""
echo "}"