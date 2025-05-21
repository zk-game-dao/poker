#!/bin/bash
#
# Compile and extract candid from all Rust packages in the src/ directory.
#

compile_and_extract() {
  local canister=$1
#   local canister_name=${canister%"_canister"}

  echo "Compiling the Canister Wasm module for $canister..."
  cargo build --release --target wasm32-unknown-unknown --package "$canister"

  if [ $? -ne 0 ]; then
    echo "Failed to compile $canister."
    exit 1
  fi

  local wasm_path="target/wasm32-unknown-unknown/release/${canister}.wasm"
  local did_path="./src/${canister}/${canister}.did"

  if [ ! -f "$wasm_path" ]; then
    echo "Error: Wasm file not found at $wasm_path"
    read -p "Would you like to skip this canister and continue? (yes/no): " skip_confirm
    if [ "$skip_confirm" == "yes" ]; then
      return
    else
      exit 1
    fi
  fi

  echo "Extracting candid from the Wasm module and saving to $did_path..."
  if ! command -v candid-extractor &> /dev/null; then
    echo "Error: candid-extractor is not installed."
    read -p "Would you like to install it now? (yes/skip/abort): " action
    case $action in
      yes)
        cargo install candid-extractor
        if [ $? -ne 0 ]; then
          echo "Failed to install candid-extractor. Please install it manually."
          exit 1
        fi
        ;;
      skip)
        echo "Skipping candid extraction for $canister."
        return
        ;;
      *)
        echo "Aborting."
        exit 1
        ;;
    esac
  fi

  candid-extractor "$wasm_path" > "$did_path"
  if [ $? -ne 0 ]; then
    echo "Failed to extract candid for $canister."
    read -p "Would you like to skip this canister and continue? (yes/no): " skip_confirm
    if [ "$skip_confirm" == "yes" ]; then
      return
    else
      exit 1
    fi
  fi

  echo "Successfully extracted candid for $canister."
}

# Find all Rust packages in the src/ directory
canisters=$(cargo metadata --no-deps --format-version 1 | jq -r '.packages[] | select(.manifest_path | contains("/src/")) | .name')

for canister in $canisters; do
  compile_and_extract "$canister"
done