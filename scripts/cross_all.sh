#!/usr/bin/env bash
set -euo pipefail

if [ $# -lt 1 ]; then
    echo "Usage: $0 <subcommand> [args...]"
    exit 1
fi

cmd=$1
shift

# Parse only base targets (ignore .env or any other suffix)
targets=$(grep '^\[target\.' Cross.toml | grep -E '^\[target\.[a-zA-Z0-9\-]+]$' | sed 's/^\[target\.//' | sed 's/\]$//')

for t in $targets; do
    echo "=== Running 'cross $cmd' for target $t ==="
    cross $cmd --target $t "$@"
done
