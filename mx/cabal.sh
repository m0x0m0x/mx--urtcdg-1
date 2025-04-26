#!/usr/bin/bash
# This bash srcript is for making cast accounts
clear

# Colors
export RED='\033[0;31m'
export GREEN='\033[0;32m'
export YELLOW='\033[0;33m'
export BLUE='\033[0;34m'
export PURPLE='\033[0;35m'
export CYAN='\033[0;36m'
export WHITE='\033[0;37m'
export NC='\033[0m' # No Color

# Commands

hea1() {
    echo -e "${CYAN}~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~${NC}"
    echo -e "${PURPLE}$1${NC}"
    echo -e "${CYAN}~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~${NC}"
}

cwb() {
    hea1 "Balance Checker"

    # --- Configuration ---
    # Wallet Addresses
    local -a wallets=(
        "0x991A0FF9529bbC4E1b66cdb47e44DEeD1FcEE999"
        "0x99F23c70837aa99175939077D34F20896CE8D399"
        "0x995D96C5f70087cd6eA3c4F5eB8Ab7DeC3fDbe99"

    )

    # Network Configurations (Format: "NetworkName:RPC_URL")
    local -a networks=(
        "Sepolia:https://eth-sepolia.g.alchemy.com/v2/y-cD2hUWMXwa6cAWy7uplLSSoRQ5v7Fx"
        "Holesky:https://eth-holesky.g.alchemy.com/v2/y-cD2hUWMXwa6cAWy7uplLSSoRQ5v7Fx"
    )
    # --- End Configuration ---

    # Check for 'cast' command dependency
    if ! command -v cast &>/dev/null; then
        echo -e "${RED}Error: 'cast' command not found. Please install Foundry (https://getfoundry.sh).${NC}"
        return 1 # Use return instead of exit for functions
    fi

    local overall_status=0 # 0 = success, 1 = failure occurred

    # Loop through each wallet
    for wallet_address in "${wallets[@]}"; do
        echo -e "--- Checking Wallet: ${YELLOW}${wallet_address}${NC} ---"

        # Loop through each network for the current wallet
        for network_info in "${networks[@]}"; do
            # Split network info into name and URL
            IFS=':' read -r network_name rpc_url <<<"$network_info"

            local balance_output
            local exit_code

            # Construct and execute the command directly
            # Using 'cast balance --ether' (or 'cast b -e') to get balance in Ether
            # Capture stderr along with stdout to see potential errors from cast/RPC
            echo "Checking ${network_name}..."
            balance_output=$(cast balance --ether "${wallet_address}" --rpc-url "${rpc_url}" 2>&1)
            exit_code=$?

            if [ $exit_code -ne 0 ]; then
                # Report error but continue checking other networks/wallets
                echo -e "${RED}Error:${NC} Failed to get ${network_name} balance."
                echo -e "${RED}Details:${NC} ${balance_output}" # Show the error message from cast
                overall_status=1                                # Mark that at least one failure occurred
            else
                # Report success
                echo -e "${GREEN}${network_name} Balance:${NC} ${balance_output} ETH"
            fi
        done
        echo "-------------------------------------------------------"
    done

    # Final status report
    if [ $overall_status -ne 0 ]; then
        echo -e "${YELLOW}Balance Check Completed with errors.${NC}"
        return 1 # Indicate failure
    else
        echo -e "${GREEN}Balance Check Completed Successfully.${NC}"
        return 0 # Indicate success
    fi
}

# Sending function

cas() {
    echo -e "${GREEN}Sending function called${NC}"

    # --- Configuration ---
    local -a wallets=(
        "0x991A0FF9529bbC4E1b66cdb47e44DEeD1FcEE999"
        "0x99F23c70837aa99175939077D34F20896CE8D399"
        "0x995D96C5f70087cd6eA3c4F5eB8Ab7DeC3fDbe99"
    )

    local -a keyz=(
        "0x15e64abfed3218cfe2ea1117e38eedb0a51990544534700e61cd803674be31ff"
        "0xe1eae1464d5fe82c12606b62ccdbe0eccb90e2d2134417b459dfb9dfda09f684"
        "0x17c674a1c7e43761479d09d76864c49d516e217006d965ae9df1fbf02ccc241d"

    )

    # Network Configurations (Format: "NetworkName:RPC_URL")
    local -a networks=(
        "https://eth-sepolia.g.alchemy.com/v2/y-cD2hUWMXwa6cAWy7uplLSSoRQ5v7Fx"
        "https://eth-holesky.g.alchemy.com/v2/y-cD2hUWMXwa6cAWy7uplLSSoRQ5v7Fx"
    )

    # --- AmountSend ---
    local amount_send=0.01 # Amount to send in ETH

    CO1="cast send --rpc-url ${networks[0]} --private-key ${keyz[0]} ${wallets[1]} --value ${amount_send}ether"

    echo -e "${GREEN}Sending ${amount_send} ETH from ${wallets[0]} to ${wallets[1]} on ${networks[0]}...${NC}"
    send_output=$(eval "$CO1" 2>&1)
    if [ $? -ne 0 ]; then
        echo -e "${RED}Error: ${send_output}${NC}"
        return 1 # Indicate failure
    fi
    echo -e "${GREEN}Transaction successful: ${send_output}${NC}"
    echo -e "${GREEN}Transaction hash: ${send_output}${NC}"
}

# Execution
cwb
