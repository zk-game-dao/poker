# Declare principal id, balance, and account id as arrays
Dealer=("uyxh5-bi3za-gxbfs-op3gj-ere73-a6jhv-5jky3-zawef-b5r2s-k26un-sae" 1000000000000000 "Dealer")
SmallBlind=("v5uia-7a6h5-5m26b-e5muk-evsay-fxtsv-aejwu-hbmej-x4qkf-lxchn-yqe" 1000000000000000 "SmallBlind")
BigBlind=("vovmm-cj7tl-3jfzz-a2ibq-s6mof-4xct2-q5o6d-nqegm-b6tqy-chvfg-uae" 1000000000000000 "BigBlind")

# Save accounts in an array of arrays
ACCOUNTS=(Dealer SmallBlind BigBlind)

for account_name in "${ACCOUNTS[@]}"; do
  # Access the arrays directly using eval to construct the array name
  eval "account=(\"\${$account_name[@]}\")"
  
  # Assign array elements to variables
  principal_id="${account[0]}"
  balance="${account[1]}"
  account_id="${account[2]}"

  dfx canister call app_backend create_user "(\"${account_id}\", null, opt(\"${principal_id}\"))"
done

# Create a table
dfx canister call app_backend create_table "(\"No limit\", 0, 8, false, 209.0, 1.0, variant{ NoLimit }, 1, 0, 0)"
dfx canister call app_backend create_table "(\"Spread limit\", 0, 8, false, 209.0, 1.0, variant{ SpreadLimit = record{1.0;2.0} }, 1, 0, 0)"
dfx canister call app_backend create_table "(\"Fixed limit\", 0, 8, false, 209.0, 1.0, variant{ FixedLimit = record{1.0;2.0} }, 1, 0, 0)"