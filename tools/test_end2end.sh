#!/usr/bin/env bash
set -euo pipefail

BASE=http://127.0.0.1:3000

# simple checks
curl -sfI "$BASE/" | head -n 1
curl -sf "$BASE/api/mood?mood=happy" | jq
curl -sf "$BASE/api/surprise" | jq

echo "End-to-end checks passed (server must be running)."
