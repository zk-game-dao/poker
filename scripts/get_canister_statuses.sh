#!/bin/bash

# Canister Status Checker
# Usage: ./canister_status.sh [mainnet|testnet] [tables]

set -e

# Colors for pretty output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
WHITE='\033[1;37m'
NC='\033[0m' # No Color

# Function to print colored output
print_colored() {
    local color=$1
    local text=$2
    echo -e "${color}${text}${NC}"
}

# Function to add underscores to numbers for readability
format_number_with_underscores() {
    local number=$1
    # Handle empty or non-numeric input
    if [[ ! $number =~ ^[0-9]+$ ]]; then
        echo "$number"
        return
    fi
    
    # Add underscores every 3 digits from right to left (compatible with macOS sed)
    echo "$number" | rev | sed 's/\([0-9]\{3\}\)/\1_/g' | sed 's/_$//' | rev
}

# Function to format cycles with both raw and human readable (like the table output)
format_cycles_detailed() {
    local cycles=$1
    # Handle empty or non-numeric input
    if [[ ! $cycles =~ ^[0-9]+$ ]]; then
        echo "0 (0 T cycles)"
        return
    fi
    
    local formatted_cycles=$(format_number_with_underscores $cycles)
    
    if [[ $cycles -ge 1000000000000 ]]; then
        local t_cycles=$(( cycles / 1000000000000 ))
        echo "${formatted_cycles} (${t_cycles} T cycles)"
    elif [[ $cycles -ge 1000000000 ]]; then
        local b_cycles=$(( cycles / 1000000000 ))
        echo "${formatted_cycles} (${b_cycles} B cycles)"
    elif [[ $cycles -ge 1000000 ]]; then
        local m_cycles=$(( cycles / 1000000 ))
        echo "${formatted_cycles} (${m_cycles} M cycles)"
    elif [[ $cycles -ge 1000 ]]; then
        local k_cycles=$(( cycles / 1000 ))
        echo "${formatted_cycles} (${k_cycles} K cycles)"
    else
        echo "${formatted_cycles} (${cycles} cycles)"
    fi
}

# Function to format bytes with both raw and human readable (like the table output)
format_bytes_detailed() {
    local bytes=$1
    # Handle empty or non-numeric input
    if [[ ! $bytes =~ ^[0-9]+$ ]]; then
        echo "0 bytes (0 MB)"
        return
    fi
    
    local formatted_bytes=$(format_number_with_underscores $bytes)
    
    if [[ $bytes -ge 1073741824 ]]; then
        local gb=$(( bytes / 1073741824 ))
        echo "${formatted_bytes} bytes (${gb} GB)"
    elif [[ $bytes -ge 1048576 ]]; then
        local mb=$(( bytes / 1048576 ))
        echo "${formatted_bytes} bytes (${mb} MB)"
    elif [[ $bytes -ge 1024 ]]; then
        local kb=$(( bytes / 1024 ))
        echo "${formatted_bytes} bytes (${kb} KB)"
    else
        echo "${formatted_bytes} bytes"
    fi
}

# Function to print section header
print_header() {
    local text=$1
    echo
    print_colored $CYAN "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    print_colored $WHITE "  $text"
    print_colored $CYAN "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo
}

# Function to format bytes to human readable
format_bytes() {
    local bytes=$1
    # Handle empty or non-numeric input
    if [[ ! $bytes =~ ^[0-9]+$ ]]; then
        echo "0B"
        return
    fi
    
    if [[ $bytes -lt 1024 ]]; then
        echo "${bytes}B"
    elif [[ $bytes -lt 1048576 ]]; then
        echo "$(( bytes / 1024 ))KB"
    elif [[ $bytes -lt 1073741824 ]]; then
        echo "$(( bytes / 1048576 ))MB"
    else
        echo "$(( bytes / 1073741824 ))GB"
    fi
}

# Function to format cycles to human readable
format_cycles() {
    local cycles=$1
    # Handle empty or non-numeric input
    if [[ ! $cycles =~ ^[0-9]+$ ]]; then
        echo "0"
        return
    fi
    
    if [[ $cycles -lt 1000 ]]; then
        echo "$cycles"
    elif [[ $cycles -lt 1000000 ]]; then
        echo "$(( cycles / 1000 ))K"
    elif [[ $cycles -lt 1000000000 ]]; then
        echo "$(( cycles / 1000000 ))M"
    elif [[ $cycles -lt 1000000000000 ]]; then
        echo "$(( cycles / 1000000000 ))B"
    else
        echo "$(( cycles / 1000000000000 ))T"
    fi
}

# Function to get status color based on status
get_status_color() {
    local status=$1
    case $status in
        "running"|"Running")
            echo $GREEN
            ;;
        "stopped"|"Stopped")
            echo $RED
            ;;
        "stopping"|"Stopping")
            echo $YELLOW
            ;;
        *)
            echo $WHITE
            ;;
    esac
}

# Function to check a single canister (for regular mode)
check_single_canister() {
    local canister_id=$1
    local network_flag=$2
    local canister_name=${3:-""}
    
    if [[ -n "$canister_name" ]]; then
        print_colored $BLUE "┌─ Checking: $canister_name ($canister_id)"
    fi
    
    # Get canister status
    status_output=$(dfx canister status $canister_id $network_flag 2>&1 || true)
    
    if echo "$status_output" | grep -q "Error\|error\|cannot find canister"; then
        print_colored $RED "└─ ❌ ERROR: $(echo "$status_output" | head -1)"
        echo
        return
    fi
    
    # Parse status information - match exact dfx output format
    status=$(echo "$status_output" | grep "^Status:" | awk '{print $2}' || echo "unknown")
    controllers=$(echo "$status_output" | grep "^Controllers:" | sed 's/^Controllers: //' || echo "unknown")
    memory_allocation=$(echo "$status_output" | grep "^Memory allocation:" | awk '{print $3}' | tr -d '_,' || echo "0")
    memory_used=$(echo "$status_output" | grep "^Memory Size:" | awk '{print $3}' | tr -d '_,' || echo "0")
    compute_allocation=$(echo "$status_output" | grep "^Compute allocation:" | awk '{print $3}' | tr -d '%' || echo "0")
    freezing_threshold=$(echo "$status_output" | grep "^Freezing threshold:" | awk '{print $3}' | tr -d '_,' || echo "0")
    cycles=$(echo "$status_output" | grep "^Balance:" | awk '{print $2}' | tr -d '_,' || echo "0")
    reserved_cycles_limit=$(echo "$status_output" | grep "^Reserved cycles limit:" | awk '{print $4}' | tr -d '_,' || echo "0")
    module_hash=$(echo "$status_output" | grep "^Module hash:" | awk '{print $3}' || echo "none")
    
    # Format the output to match table style
    status_color=$(get_status_color $status)
    
    print_colored $CYAN "├─ 📊 Canister Status Report"
    print_colored $CYAN "├─ ════════════════════════════════════════════════════════════════"
    print_colored $WHITE "├─ 🆔 Canister ID: $canister_id"
    print_colored $status_color "├─ 🔄 Status: $status"
    
    if [[ $memory_used != "0" ]]; then
        formatted_memory_used=$(format_bytes_detailed $memory_used)
        print_colored $WHITE "├─ 💾 Memory Size: $formatted_memory_used"
    fi
    
    if [[ $cycles != "0" ]]; then
        formatted_cycles=$(format_cycles_detailed $cycles)
        print_colored $PURPLE "├─ ⚡ Cycles: $formatted_cycles"
    fi
    
    # Show controllers (truncated if too long)
    if [[ ${#controllers} -gt 60 ]]; then
        short_controllers=$(echo "$controllers" | cut -c1-57)...
        print_colored $YELLOW "├─ 🎛️  Controllers: $short_controllers"
    else
        print_colored $YELLOW "├─ 🎛️  Controllers: $controllers"
    fi
    
    print_colored $WHITE "├─ 📈 Compute Allocation: $compute_allocation"
    
    if [[ $memory_allocation != "0" ]]; then
        formatted_memory_allocation=$(format_bytes_detailed $memory_allocation)
        print_colored $WHITE "├─ 🧠 Memory Allocation: $formatted_memory_allocation"
    else
        print_colored $WHITE "├─ 🧠 Memory Allocation: 0 bytes"
    fi
    
    formatted_freezing_threshold=$(format_number_with_underscores $freezing_threshold)
    print_colored $WHITE "├─ 🧊 Freezing Threshold: $formatted_freezing_threshold"
    
    if [[ $reserved_cycles_limit != "0" ]]; then
        formatted_reserved_cycles=$(format_number_with_underscores $reserved_cycles_limit)
        print_colored $WHITE "├─ 📊 Reserved Cycles Limit: $formatted_reserved_cycles"
    fi
    
    if [[ $module_hash != "none" ]]; then
        short_hash=$(echo $module_hash | cut -c1-12)
        print_colored $YELLOW "├─ 🔧 Module Hash: ${short_hash}..."
    fi
    
    print_colored $CYAN "├─ ════════════════════════════════════════════════════════════════"
    print_colored $WHITE "└─ Canister ID: $canister_id"
    echo
}

# Function to check a single table canister using its custom endpoint
check_table_canister() {
    local canister_id=$1
    local network_flag=$2
    local counter=${3:-""}
    
    if [[ -n "$counter" ]]; then
        print_colored $BLUE "┌─ Checking Table #$counter: $canister_id"
    else
        print_colored $BLUE "┌─ Checking Table: $canister_id"
    fi
    
    # Call the custom get_canister_status_formatted method
    status_output=$(dfx canister call $canister_id get_canister_status_formatted $network_flag 2>&1 || true)
    
    if echo "$status_output" | grep -q "Error\|error\|cannot find canister"; then
        print_colored $RED "└─ ❌ ERROR: $(echo "$status_output" | head -1)"
        echo
        return
    fi
    
    # Extract the formatted status from the Candid response
    # The format is: ( variant { 17_724 = "formatted_string" }, )
    formatted_status=$(echo "$status_output" | grep -o '"📊[^"]*"' | sed 's/^"//;s/"$//' | sed 's/\\n/\n/g')
    
    if [[ -n "$formatted_status" ]]; then
        # Print the formatted status with proper indentation
        echo "$formatted_status" | while IFS= read -r line; do
            if [[ "$line" == "📊 Canister Status Report" ]]; then
                print_colored $CYAN "├─ $line"
            elif [[ "$line" == "════════════════════════════════════════════════════════════════" ]]; then
                print_colored $CYAN "├─ $line"
            elif [[ "$line" =~ ^🆔 ]]; then
                print_colored $WHITE "├─ $line"
            elif [[ "$line" =~ ^🔄 ]]; then
                status_value=$(echo "$line" | sed 's/🔄 Status: //')
                status_color=$(get_status_color "$status_value")
                print_colored $status_color "├─ $line"
            elif [[ "$line" =~ ^💾 ]]; then
                print_colored $WHITE "├─ $line"
            elif [[ "$line" =~ ^⚡ ]]; then
                print_colored $PURPLE "├─ $line"
            elif [[ "$line" =~ ^🎛️ ]]; then
                print_colored $YELLOW "├─ $line"
            elif [[ "$line" =~ ^📈|^🧠|^🧊|^📊 ]]; then
                print_colored $WHITE "├─ $line"
            elif [[ -n "$line" ]]; then
                print_colored $WHITE "├─ $line"
            fi
        done
    else
        print_colored $RED "├─ ❌ Failed to parse canister status response"
    fi
    
    print_colored $WHITE "└─ Canister ID: $canister_id"
    echo
}

# Function to check table canisters
check_table_canisters() {
    local network_name=$1
    local network_flag=$2
    
    # Get table_index canister ID from canister_ids.json
    local table_index_id=$(cat $CANISTER_FILE | jq -r '.table_index.ic // empty')
    
    if [[ -z "$table_index_id" ]]; then
        print_colored $RED "Error: table_index canister ID not found in $CANISTER_FILE"
        exit 1
    fi
    
    print_header "🎯 TABLE CANISTERS STATUS - $(echo $network_name | tr '[:lower:]' '[:upper:]')"
    print_colored $BLUE "📋 Fetching table principals from table_index: $table_index_id"
    echo
    
    # Call the get_all_table_principals method
    table_principals_output=$(dfx canister call $table_index_id get_all_table_principals $network_flag 2>&1 || true)
    
    if echo "$table_principals_output" | grep -q "Error\|error"; then
        print_colored $RED "❌ Error fetching table principals: $table_principals_output"
        exit 1
    fi
    
    # Parse the principals from the output (format: (Ok (vec { principal "xxx"; principal "yyy"; })))
    table_principals=$(echo "$table_principals_output" | grep -o 'principal "[^"]*"' | sed 's/principal "//g' | sed 's/"//g')
    
    if [[ -z "$table_principals" ]]; then
        print_colored $YELLOW "⚠️  No table principals found"
        return
    fi
    
    # Count the principals
    table_count=$(echo "$table_principals" | wc -l | tr -d ' ')
    print_colored $GREEN "✅ Found $table_count table canisters"
    echo
    
    # Check each table canister
    local counter=1
    while IFS= read -r principal; do
        if [[ -n "$principal" ]]; then
            check_table_canister "$principal" "$network_flag" "$counter"
            ((counter++))
        fi
    done <<< "$table_principals"
    
    print_colored $GREEN "✅ Table status check complete!"
}

# ==================== ARGUMENT PARSING ====================

# Check arguments
if [[ $# -eq 0 ]]; then
    print_colored $RED "Usage: $0 [mainnet|testnet] [tables]"
    print_colored $YELLOW "  mainnet/testnet: Check canisters from canister_ids.json or test_canister_ids.json"
    print_colored $YELLOW "  tables: Check all table canisters from table_index"
    exit 1
fi

NETWORK="$1"
MODE="$2"
CANISTER_FILE=""
NETWORK_FLAG=""

case $NETWORK in
    "mainnet")
        CANISTER_FILE="canister_ids.json"
        NETWORK_FLAG="--network ic"
        ;;
    "testnet")
        CANISTER_FILE="test_canister_ids.json"
        NETWORK_FLAG="--network local"
        ;;
    *)
        print_colored $RED "Invalid network. Use 'mainnet' or 'testnet'"
        exit 1
        ;;
esac

# Check if canister file exists (only needed for regular mode)
if [[ "$MODE" != "tables" && ! -f $CANISTER_FILE ]]; then
    print_colored $RED "Error: $CANISTER_FILE not found!"
    exit 1
fi

# Check if dfx is installed
if ! command -v dfx &> /dev/null; then
    print_colored $RED "Error: dfx is not installed or not in PATH"
    exit 1
fi

# ==================== MAIN EXECUTION ====================

# Main execution logic
if [[ "$MODE" == "tables" ]]; then
    # Check table canisters mode
    check_table_canisters $NETWORK "$NETWORK_FLAG"
else
    # Regular canister checking mode
    # Print main header
    print_header "🚀 CANISTER STATUS REPORT - $(echo $NETWORK | tr '[:lower:]' '[:upper:]')"

    # Extract canister IDs and names from JSON
    canister_data=$(cat $CANISTER_FILE | jq -r 'to_entries[] | "\(.key):\(.value.ic)"')

    # Check each canister
    while IFS=':' read -r canister_name canister_id; do
        check_single_canister "$canister_id" "$NETWORK_FLAG" "$canister_name"
    done <<< "$canister_data"
    
    print_colored $GREEN "✅ Status check complete!"
fi

echo
