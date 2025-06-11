#!/bin/bash
set -e

if [ -z "$1" ]; then
  echo "❌ Usage: $0 <proposal_id>"
  exit 1
fi

PROPOSAL_ID=$1

echo "🗳️ Voting on proposal $PROPOSAL_ID..."
dfx canister call governance vote "(record {proposal_id = $PROPOSAL_ID; vote = true})"

echo "🚀 Executing proposal $PROPOSAL_ID..."
dfx canister call governance execute_proposal "(record {proposal_id = $PROPOSAL_ID})"

echo "✅ Proposal $PROPOSAL_ID finished."
