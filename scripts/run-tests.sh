#!/bin/bash

POCKET_IC_BIN=$(pwd)/tests/pocket-ic
POCKET_IC_SERVER_VERSION="5.0.0"

NNS_WASM_PATH="tests/wasms"
WASM_URL="https://download.dfinity.systems/ic/0a51fd74f08b2e6f23d6e1d60f1f52eb73b40ccc/canisters"

if [[ $OSTYPE == "linux-gnu"* ]] || [[ $RUNNER_OS == "Linux" ]]
then
    PLATFORM=linux
elif [[ $OSTYPE == "darwin"* ]] || [[ $RUNNER_OS == "macOS" ]]
then
    PLATFORM=darwin
else
    echo "OS not supported: ${OSTYPE:-$RUNNER_OS}"
    exit 1
fi

download_nns_wasm() {
  local canister_name=$1
  local file_name=$2

  if [ -f $NNS_WASM_PATH/$canister_name.wasm.gz ]; then
    return
  fi

  mkdir -p $NNS_WASM_PATH

  HTTP_CODE=$(curl -so $NNS_WASM_PATH/$canister_name.wasm.gz $WASM_URL/$file_name.wasm.gz --write-out "%{http_code}")
  echo $WASM_URL/$canister_name.wasm.gz

  if [[ ${HTTP_CODE} -ne 200 ]] ; then
    echo "Failed to download wasm. Response code: ${HTTP_CODE}"
    exit 1
  fi

  echo "$canister_name wasm downloaded"
}

if ! [ -f $POCKET_IC_BIN ]; then
  cd libraries/testing
  echo "PocketIC download starting"
  curl -Ls https://github.com/dfinity/pocketic/releases/download/${POCKET_IC_SERVER_VERSION}/pocket-ic-x86_64-${PLATFORM}.gz -o pocket-ic.gz || exit 1
  gzip -df pocket-ic.gz
  chmod +x pocket-ic
  mv pocket-ic ./tests
  echo "PocketIC download completed"
fi

./scripts/build-canisters.sh

# Download nns canisters
download_nns_wasm icp_ledger ledger-canister

export POCKET_IC_BIN

# Check if a test name is passed as an argument
if [ -n "$1" ]; then
  TEST_NAME="$1"
  RUST_TEST_THREADS=1 cargo test --package tests "$TEST_NAME" -- --test-threads=1 --nocapture
else
  RUST_TEST_THREADS=1 cargo test --package tests -- --test-threads=1 --nocapture
fi
