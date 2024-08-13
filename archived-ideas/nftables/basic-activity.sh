#!/bin/bash
nft flush ruleset

# Create a new table
nft add table inet filter

# Create a chain for logging
nft add chain inet filter log_chain { type filter hook prerouting priority 0 \; }

# Log new connections
nft add rule inet filter log_chain ct state new log prefix \"[NFTABLES] NEW \"

# Log established connections
nft add rule inet filter log_chain ct state established log prefix \"[NFTABLES] ESTABLISHED \"

# Log related connections
nft add rule inet filter log_chain ct state related log prefix \"[NFTABLES] RELATED \"

# Log invalid connections
nft add rule inet filter log_chain ct state invalid log prefix \"[NFTABLES] INVALID \"
